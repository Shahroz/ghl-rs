#!/usr/bin/env python3
"""Generate the full API reference for ghl-rs from HighLevel's OpenAPI specs.

Usage:
    python3 xtask/generate_api_docs.py <highlevel-api-docs-checkout> docs/api

Produces, per API module, a Markdown page documenting every endpoint (path and
query parameters, request-body fields, required OAuth scopes, `Version` header),
every data model with its field types, and every enum with its allowed values —
plus copy-pasteable Rust and MCP call examples. `docs/api/README.md` indexes it
all.

Large enums repeated across many schemas (country and timezone lists) are
collapsed into `docs/api/shared-enums.md` so module pages stay readable.
"""
import json
import pathlib
import re
import sys
from collections import defaultdict

# Reuse the service generator so every endpoint can name its Rust method.
sys.path.insert(0, str(pathlib.Path(__file__).parent))
from generate_services import HANDWRITTEN, ServiceGen  # noqa: E402

METHODS = ("get", "post", "put", "patch", "delete")
# Enums with more members than this get hoisted into the shared page.
BIG_ENUM = 30

# Which modules have hand-written typed SDK services (and their method names).
TYPED_SERVICES = {
    "contacts": ("ghl.contacts()", ["create", "get", "update", "delete", "list"]),
    "opportunities": (
        "ghl.opportunities()",
        ["pipelines", "create", "get", "update", "update_status", "delete", "search"],
    ),
    "conversations": ("ghl.conversations()", ["search", "messages", "send_message"]),
    "calendars": (
        "ghl.calendars()",
        ["list", "free_slots", "create_appointment", "get_appointment"],
    ),
    "locations": ("ghl.locations()", ["get", "search"]),
}

# MCP tools that cover a module directly.
MCP_TOOLS = {
    "contacts": [
        "ghl_search_contacts", "ghl_get_contact", "ghl_create_contact",
        "ghl_update_contact", "ghl_delete_contact",
    ],
    "opportunities": [
        "ghl_list_pipelines", "ghl_search_opportunities", "ghl_get_opportunity",
        "ghl_create_opportunity", "ghl_move_opportunity",
    ],
    "conversations": ["ghl_search_conversations", "ghl_get_messages", "ghl_send_message"],
    "calendars": ["ghl_list_calendars", "ghl_get_free_slots", "ghl_book_appointment"],
    "locations": ["ghl_list_locations"],
}


def clean(text, limit=400):
    if not text:
        return ""
    t = re.sub(r"<br\s*/?>", " ", str(text))
    t = re.sub(r"\s+", " ", t).strip()
    t = t.replace("|", "\\|")
    return t[:limit] + ("…" if len(t) > limit else "")


def anchor(s):
    return re.sub(r"[^a-z0-9]+", "-", s.lower()).strip("-")


def rust_type_hint(schema, spec_schemas):
    """Short human type for docs (mirrors what ghl-models generates)."""
    if not isinstance(schema, dict):
        return "unknown"
    for key in ("allOf", "oneOf", "anyOf"):
        if schema.get(key):
            branches = schema[key]
            if key == "allOf" and len(branches) == 1 and "$ref" in branches[0]:
                return branches[0]["$ref"].split("/")[-1]
            return "JSON"
    if "$ref" in schema:
        return schema["$ref"].split("/")[-1]
    if schema.get("enum"):
        return "String (enum)"
    t = schema.get("type")
    if t == "array":
        return f"Vec<{rust_type_hint(schema.get('items') or {}, spec_schemas)}>"
    if t == "object":
        return "JSON"
    return {
        "string": "String", "boolean": "bool",
        "integer": "i64", "number": "f64",
    }.get(t, "JSON")


class ModuleDoc:
    def __init__(self, module):
        self.module = module
        self.versions = {}       # api_version -> {"ops": [...], "schemas": {...}}

    def add(self, api_version, spec):
        ops = []
        for path, item in (spec.get("paths") or {}).items():
            path_params = item.get("parameters") or []
            for method in METHODS:
                op = item.get(method)
                if not op:
                    continue
                ops.append(self._op(method, path, op, path_params, api_version))
        ops.sort(key=lambda o: (o["path"], o["method"]))
        self.versions[api_version] = {
            "ops": ops,
            "schemas": (spec.get("components") or {}).get("schemas") or {},
        }

    @staticmethod
    def _op(method, path, op, path_params, api_version):
        params = []
        for p in list(path_params) + list(op.get("parameters") or []):
            if "$ref" in p or p.get("in") not in ("path", "query"):
                continue
            sch = p.get("schema") or {}
            params.append({
                "name": p.get("name"),
                "in": p.get("in"),
                "required": bool(p.get("required")) or p.get("in") == "path",
                "type": sch.get("type", "string"),
                "enum": sch.get("enum"),
                "desc": clean(p.get("description"), 160),
            })
        version = None
        for p in list(op.get("parameters") or []) + list(path_params):
            if p.get("name") == "Version":
                sch = p.get("schema") or {}
                version = (sch.get("enum") or [sch.get("default")])[0]
        scopes = []
        for sec in op.get("security") or []:
            for _, sc in sec.items():
                scopes.extend(sc or [])

        body = None
        rb = (op.get("requestBody") or {}).get("content") or {}
        js = rb.get("application/json") or {}
        bs = js.get("schema") or {}
        if "$ref" in bs:
            body = {"ref": bs["$ref"].split("/")[-1]}
        elif bs.get("properties"):
            body = {
                "fields": list(bs["properties"].keys()),
                "required": bs.get("required") or [],
            }

        # Success response schema ref, when the spec names one.
        resp_ref = None
        for code, r in (op.get("responses") or {}).items():
            if str(code).startswith("2"):
                sch = (((r.get("content") or {}).get("application/json") or {}).get("schema") or {})
                if "$ref" in sch:
                    resp_ref = sch["$ref"].split("/")[-1]
                break

        return {
            "method": method.upper(),
            "path": path,
            "summary": clean(op.get("summary") or op.get("operationId"), 200),
            "desc": clean(op.get("description"), 500),
            "params": params,
            "body": body,
            "response": resp_ref,
            "scopes": sorted(set(scopes)),
            "version": version or ("v3" if api_version == "v3" else None),
            "api_version": api_version,
        }

    def op_id(self, op):
        parts = [p for p in op["path"].strip("/").split("/") if p]
        cleaned = ["by_" + p.strip("{}") if p.startswith("{") else re.sub(r"[^a-zA-Z0-9]+", "_", p) for p in parts]
        base = f"{self.module}.{op['method'].lower()}_" + "_".join(cleaned)
        return ("v3:" if op["api_version"] == "v3" else "") + base


def render_module(md, shared_enums, rust_methods=None):
    rust_methods = rust_methods or {}
    m = md.module
    out = [f"# `{m}`", ""]

    v2 = md.versions.get("v2")
    v3 = md.versions.get("v3")
    counts = []
    if v2:
        counts.append(f"**{len(v2['ops'])}** operations / **{len(v2['schemas'])}** models in API v2")
    if v3:
        counts.append(f"**{len(v3['ops'])}** operations / **{len(v3['schemas'])}** models in API v3")
    out.append(" · ".join(counts))
    out.append("")

    # How to call it
    out.append("## How to call it")
    out.append("")
    if rust_methods:
        n = len(rust_methods)
        rust_mod = re.sub(r"[^a-z0-9]+", "_", m)
        out.append(f"**Every endpoint has a typed Rust method.** Enable the `{m}` "
                   f"cargo feature on `ghl-sdk`, then call any of the {n} generated "
                   f"methods on `ghl.{rust_mod}()`:")
        out.append("")
        out.append("```toml")
        out.append(f'ghl-sdk = {{ version = "0.4", features = ["{m}"] }}')
        out.append("```")
        out.append("")
        if m in TYPED_SERVICES:
            svc, methods = TYPED_SERVICES[m]
            out.append(f"This module also has hand-written ergonomic helpers on the same "
                       f"`{svc}`: " + ", ".join(f"`{x}()`" for x in methods)
                       + " (envelope unwrapping, paginated `Stream`s).")
            out.append("")
        if m in MCP_TOOLS:
            out.append("MCP tools: " + ", ".join(f"`{t}`" for t in MCP_TOOLS.get(m, [])) + ".")
            out.append("")
    elif m in TYPED_SERVICES:
        svc, methods = TYPED_SERVICES[m]
        out.append(f"This module has a **typed SDK service**: `{svc}` with "
                   + ", ".join(f"`{x}()`" for x in methods) + ".")
        out.append("")
        out.append("MCP tools: " + ", ".join(f"`{t}`" for t in MCP_TOOLS.get(m, [])) + ".")
    else:
        out.append("No hand-written service yet — reach these endpoints two ways:")
        out.append("")
        out.append("**From Rust** (typed body via [`ghl-models`](https://docs.rs/ghl-models)):")
        out.append("")
        out.append("```rust,ignore")
        out.append(f'// cargo add ghl-models --features {m}')
        out.append(f"use ghl_models::v2::{re.sub(r'[^a-z0-9]+', '_', m)}::*;")
        out.append("")
        out.append('let body = serde_json::to_value(/* a Create…Dto from above */)?;')
        out.append('let out = ghl.request_raw("POST", "/path/", &[], Some(&body), None).await?;')
        out.append("```")
        out.append("")
        out.append("**From an AI agent** (MCP meta-tools):")
        out.append("")
        out.append("```json")
        out.append(json.dumps({"name": "ghl_search_operations",
                               "arguments": {"query": "", "module": m}}, indent=2, ensure_ascii=False))
        out.append("```")
    out.append("")

    for api_version in ("v2", "v3"):
        block = md.versions.get(api_version)
        if not block:
            continue
        out.append(f"## Endpoints — API {api_version}")
        out.append("")
        show_rust = api_version == "v2" and bool(rust_methods)
        if show_rust:
            out.append("| Method | Path | Summary | Rust method | Operation id |")
            out.append("|---|---|---|---|---|")
        else:
            out.append("| Method | Path | Summary | Operation id |")
            out.append("|---|---|---|---|")
        for op in block["ops"]:
            row = f"| `{op['method']}` | `{op['path']}` | {op['summary'] or '—'} |"
            if show_rust:
                rmm = rust_methods.get((op["method"], op["path"]))
                row += f" `{rmm[0]}()` |" if rmm else " — |"
            row += f" `{md.op_id(op)}` |"
            out.append(row)
        out.append("")

        out.append(f"### Endpoint details — {api_version}")
        out.append("")
        for op in block["ops"]:
            oid = md.op_id(op)
            out.append(f"#### `{op['method']} {op['path']}`")
            out.append("")
            if op["summary"]:
                out.append(f"**{op['summary']}**")
                out.append("")
            if op["desc"] and op["desc"] != op["summary"]:
                out.append(op["desc"])
                out.append("")
            rm = rust_methods.get((op["method"], op["path"])) if api_version == "v2" else None
            meta = [f"Operation id: `{oid}`"]
            if op["version"]:
                meta.append(f"`Version: {op['version']}`")
            if op["scopes"]:
                meta.append("Scopes: " + ", ".join(f"`{s}`" for s in op["scopes"]))
            out.append(" · ".join(meta))
            out.append("")

            path_p = [p for p in op["params"] if p["in"] == "path"]
            query_p = [p for p in op["params"] if p["in"] == "query"]
            for label, group in (("Path parameters", path_p), ("Query parameters", query_p)):
                if not group:
                    continue
                out.append(f"*{label}*")
                out.append("")
                out.append("| Name | Type | Required | Description |")
                out.append("|---|---|---|---|")
                for p in group:
                    t = p["type"]
                    if p["enum"]:
                        vals = [str(v) for v in p["enum"]]
                        if len(vals) > BIG_ENUM:
                            shared_enums[tuple(sorted(vals))].add(f"{m}.{p['name']}")
                            t = f"enum ({len(vals)} values — see [shared enums](shared-enums.md))"
                        else:
                            t = "enum: " + ", ".join(f"`{v}`" for v in vals)
                    req = "**yes**" if p["required"] else "no"
                    out.append(f"| `{p['name']}` | {t} | {req} | {p['desc'] or '—'} |")
                out.append("")

            if op["body"]:
                if op["body"].get("ref"):
                    ref = op["body"]["ref"]
                    out.append(f"*Request body*: [`{ref}`](#{anchor(ref)})")
                else:
                    req = set(op["body"].get("required") or [])
                    fields = op["body"]["fields"]
                    shown = ", ".join(
                        f"`{f}`" + ("**\\***" if f in req else "") for f in fields[:25]
                    )
                    out.append(f"*Request body fields*: {shown}"
                               + (" …" if len(fields) > 25 else "")
                               + ("  (**\\*** = required)" if req else ""))
                out.append("")
            if op["response"]:
                out.append(f"*Response*: [`{op['response']}`](#{anchor(op['response'])})")
                out.append("")

            if rm:
                fn, params_ty, svc, rust_mod = rm
                arg_bits = [f"&{p['name']}" for p in path_p]
                if params_ty:
                    arg_bits.append("&params")
                if op["body"]:
                    arg_bits.append("&body")
                out.append("*Rust*:")
                out.append("")
                out.append("```rust,ignore")
                if params_ty:
                    req_q = [q for q in query_p if q["required"]]
                    ctor = ", ".join(f"\"{q['name']}\"" for q in req_q)
                    out.append(f"use ghl_sdk::services::{rust_mod}::{params_ty};")
                    out.append("")
                    out.append(f"let params = {params_ty}::new({ctor});")
                out.append(f"let out = ghl.{rust_mod}().{fn}({', '.join(arg_bits)}).await?;")
                out.append("```")
                out.append("")
            # Ready-to-run MCP call
            args = {"operation_id": oid}
            if path_p:
                args["path_params"] = {p["name"]: f"<{p['name']}>" for p in path_p}
            req_q = [p for p in query_p if p["required"]]
            if req_q:
                args["query"] = {p["name"]: f"<{p['name']}>" for p in req_q}
            if op["body"]:
                args["body"] = {"<field>": "<value>"}
            out.append("<details><summary>MCP call</summary>")
            out.append("")
            out.append("```json")
            out.append(json.dumps({"name": "ghl_execute_operation", "arguments": args}, indent=2, ensure_ascii=False))
            out.append("```")
            out.append("")
            out.append("</details>")
            out.append("")

    # Models
    for api_version in ("v2", "v3"):
        block = md.versions.get(api_version)
        if not block or not block["schemas"]:
            continue
        rust_mod = re.sub(r"[^a-z0-9]+", "_", m)
        out.append(f"## Data models — API {api_version}")
        out.append("")
        out.append(f"In Rust: `ghl_models::{api_version}::{rust_mod}::*` "
                   f"(enable the `{m}` feature). Full field docs on "
                   f"[docs.rs](https://docs.rs/ghl-models/latest/ghl_models/{api_version}/{rust_mod}/).")
        out.append("")
        for name, schema in sorted(block["schemas"].items()):
            out.append(f"### `{name}`")
            out.append("")
            if schema.get("description"):
                out.append(clean(schema["description"]))
                out.append("")
            if schema.get("enum"):
                vals = [str(v) for v in schema["enum"]]
                out.append("String enum. Allowed values: "
                           + ", ".join(f"`{v}`" for v in vals[:BIG_ENUM])
                           + (" …" if len(vals) > BIG_ENUM else ""))
                out.append("")
                continue
            props = schema.get("properties") or {}
            if not props:
                out.append("_No fields defined in the spec._")
                out.append("")
                continue
            required = set(schema.get("required") or [])
            out.append("| Field | Type | Required | Description |")
            out.append("|---|---|---|---|")
            for pn, pd in props.items():
                t = rust_type_hint(pd, block["schemas"])
                if pd.get("enum"):
                    vals = [str(v) for v in pd["enum"]]
                    if len(vals) > BIG_ENUM:
                        shared_enums[tuple(sorted(vals))].add(f"{m}.{name}.{pn}")
                        t = f"String — {len(vals)} values ([shared](shared-enums.md))"
                    else:
                        t = "String — " + ", ".join(f"`{v}`" for v in vals)
                elif t not in ("String", "bool", "i64", "f64", "JSON") and not t.startswith("Vec<"):
                    t = f"[`{t}`](#{anchor(t)})"
                out.append(f"| `{pn}` | {t} | {'**yes**' if pn in required else 'no'} "
                           f"| {clean(pd.get('description'), 200) or '—'} |")
            out.append("")

    return "\n".join(out)


def main():
    docs_root = pathlib.Path(sys.argv[1])
    out_dir = pathlib.Path(sys.argv[2])
    out_dir.mkdir(parents=True, exist_ok=True)

    # module -> {(METHOD, path): (rust_fn, params_ty, service, rust_mod)}
    rust_methods = {}
    for f in sorted((docs_root / "apps").glob("*.json")):
        spec = json.loads(f.read_text())
        if not (spec.get("paths") or {}):
            continue
        g = ServiceGen(f.stem, spec)
        rust_methods[f.stem] = {
            (o["http"], o["path"]): (
                o["fn"],
                o["params_ty"] if o["query"] else None,
                g.svc_name(),
                g.rust_mod,
            )
            for o in g.ops
        }

    mods = {}
    for api_version, sub in (("v2", "apps"), ("v3", "apps/v3")):
        for f in sorted((docs_root / sub).glob("*.json")):
            module = f.stem.replace("-v3", "")
            spec = json.loads(f.read_text())
            mods.setdefault(module, ModuleDoc(module)).add(api_version, spec)

    shared_enums = defaultdict(set)
    index_rows = []
    total_ops = total_models = 0
    for module, md in sorted(mods.items()):
        page = render_module(md, shared_enums, rust_methods.get(module, {}))
        (out_dir / f"{module}.md").write_text(page + "\n")
        ops = sum(len(b["ops"]) for b in md.versions.values())
        models = sum(len(b["schemas"]) for b in md.versions.values())
        total_ops += ops
        total_models += models
        index_rows.append((module, md, ops, models))

    # shared enums page
    lines = ["# Shared enums", "",
             "Large value lists that repeat across many schemas, hoisted here so the",
             "module pages stay readable.", ""]
    for vals, sites in sorted(shared_enums.items(), key=lambda kv: -len(kv[0])):
        sample = sorted(sites)[:6]
        lines.append(f"## {len(vals)} values")
        lines.append("")
        lines.append("Used by: " + ", ".join(f"`{s}`" for s in sample)
                     + (f" and {len(sites) - len(sample)} more" if len(sites) > len(sample) else ""))
        lines.append("")
        lines.append("```")
        lines.append(", ".join(vals))
        lines.append("```")
        lines.append("")
    (out_dir / "shared-enums.md").write_text("\n".join(lines) + "\n")

    # index
    idx = [
        "# GoHighLevel API reference",
        "",
        f"Generated from [HighLevel's official OpenAPI specs]"
        f"(https://github.com/GoHighLevel/highlevel-api-docs) — "
        f"**{total_ops} operations** and **{total_models} data models** across "
        f"**{len(mods)} modules**, covering API v2 and v3.",
        "",
        "New here? Start with the [usage guide](../GUIDE.md). Struct field docs are also "
        "browsable on [docs.rs](https://docs.rs/ghl-models).",
        "",
        "**Legend** — 🦀 has a typed SDK service · 🤖 has dedicated MCP tools · "
        "otherwise reachable via `request_raw` / the MCP meta-tools.",
        "",
        "| Module | Ops | Models | v2 | v3 | Access |",
        "|---|---|---|---|---|---|",
    ]
    for module, md, ops, models in index_rows:
        badges = []
        if module in TYPED_SERVICES:
            badges.append("🦀")
        if module in MCP_TOOLS:
            badges.append("🤖")
        idx.append(
            f"| [`{module}`]({module}.md) | {ops} | {models} "
            f"| {'✅' if 'v2' in md.versions else '—'} "
            f"| {'✅' if 'v3' in md.versions else '—'} "
            f"| {' '.join(badges) if badges else 'meta-tools'} |"
        )
    idx += ["", "See also [shared enums](shared-enums.md) for the large repeated value lists."]
    (out_dir / "README.md").write_text("\n".join(idx) + "\n")

    print(f"wrote {len(mods)} module pages + index + shared enums -> {out_dir}")
    print(f"  {total_ops} operations, {total_models} models documented")
    size = sum(f.stat().st_size for f in out_dir.glob("*.md"))
    print(f"  total {size/1024:.0f} KiB across {len(list(out_dir.glob('*.md')))} files")


if __name__ == "__main__":
    main()
