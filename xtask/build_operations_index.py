#!/usr/bin/env python3
"""Build a compact operations index for ghl-mcp from GoHighLevel's official OpenAPI specs.

Output: crates/ghl-mcp/operations.json — one entry per endpoint, holding only
what the meta-tools need (id, module, method, path, summary, params, scopes,
Version header). Full schemas stay out; agents fetch details via describe.
"""
import json
import pathlib
import re
import sys

SPEC_DIR = pathlib.Path(sys.argv[1])
OUT = pathlib.Path(sys.argv[2])

METHODS = ("get", "post", "put", "patch", "delete")


def op_id(module: str, method: str, path: str) -> str:
    """Stable, readable id: module.method_path_segments"""
    parts = [p for p in path.strip("/").split("/") if p]
    cleaned = []
    for p in parts:
        if p.startswith("{"):
            cleaned.append("by_" + p.strip("{}"))
        else:
            cleaned.append(re.sub(r"[^a-zA-Z0-9]+", "_", p))
    return f"{module}.{method}_" + "_".join(cleaned) if cleaned else f"{module}.{method}"


def collect_params(spec_params, path_level):
    out = []
    for p in list(path_level or []) + list(spec_params or []):
        if "$ref" in p:
            continue
        loc = p.get("in")
        if loc not in ("path", "query"):
            continue
        schema = p.get("schema") or {}
        entry = {
            "name": p.get("name"),
            "in": loc,
            "required": bool(p.get("required")) or loc == "path",
            "type": schema.get("type", "string"),
        }
        if p.get("description"):
            entry["desc"] = re.sub(r"\s+", " ", p["description"])[:160]
        out.append(entry)
    return out


def body_fields(op):
    """Top-level request body field names + required list, no nested schemas."""
    rb = op.get("requestBody") or {}
    content = rb.get("content") or {}
    js = content.get("application/json") or {}
    schema = js.get("schema") or {}
    if "$ref" in schema:
        return {"ref": schema["$ref"].split("/")[-1]}
    props = schema.get("properties") or {}
    if not props:
        return None
    return {
        "fields": sorted(props.keys())[:40],
        "required": schema.get("required", [])[:20],
    }


def scopes_of(op):
    out = []
    for sec in op.get("security") or []:
        for _, scopes in sec.items():
            out.extend(scopes or [])
    return sorted(set(out))[:6]


def version_header(op, path_level):
    for p in list(op.get("parameters") or []) + list(path_level or []):
        if p.get("name") == "Version":
            schema = p.get("schema") or {}
            enum = schema.get("enum") or []
            if enum:
                return enum[0]
            if schema.get("default"):
                return schema["default"]
    return None


operations = []
modules = {}

for spec_file in sorted(SPEC_DIR.glob("*.json")):
    module = spec_file.stem
    try:
        spec = json.loads(spec_file.read_text())
    except json.JSONDecodeError as e:
        print(f"!! skipping {module}: {e}", file=sys.stderr)
        continue

    servers = spec.get("servers") or []
    base = servers[0].get("url", "") if servers else ""
    count = 0

    for path, item in (spec.get("paths") or {}).items():
        path_params = item.get("parameters") or []
        for method in METHODS:
            op = item.get(method)
            if not op:
                continue
            summary = (op.get("summary") or op.get("operationId") or "").strip()
            desc = (op.get("description") or "").strip()
            entry = {
                "id": op_id(module, method, path),
                "module": module,
                "method": method.upper(),
                "path": path,
                "summary": summary[:200],
            }
            if desc and desc != summary:
                entry["desc"] = re.sub(r"\s+", " ", desc)[:300]
            params = collect_params(op.get("parameters"), path_params)
            if params:
                entry["params"] = params
            bf = body_fields(op)
            if bf:
                entry["body"] = bf
            sc = scopes_of(op)
            if sc:
                entry["scopes"] = sc
            v = version_header(op, path_params)
            if v:
                entry["version"] = v
            if base and "leadconnectorhq" not in base:
                entry["base"] = base
            operations.append(entry)
            count += 1
    modules[module] = count

operations.sort(key=lambda o: o["id"])
payload = {
    "source": "https://github.com/GoHighLevel/highlevel-api-docs (official OpenAPI specs)",
    "operation_count": len(operations),
    "modules": modules,
    "operations": operations,
}
OUT.write_text(json.dumps(payload, separators=(",", ":")))
print(f"{len(operations)} operations across {len(modules)} modules -> {OUT}")
print(f"size: {OUT.stat().st_size/1024:.0f} KiB")
top = sorted(modules.items(), key=lambda kv: -kv[1])[:10]
print("largest modules:", ", ".join(f"{m}={n}" for m, n in top))
