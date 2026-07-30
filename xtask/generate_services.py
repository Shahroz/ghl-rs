#!/usr/bin/env python3
"""Generate typed service methods for every GoHighLevel API v2 operation.

Usage:
    python3 xtask/generate_services.py <highlevel-api-docs-checkout> crates/ghl-sdk

Emits `crates/ghl-sdk/src/services/<module>.rs`, one service struct per API
module with one async method per operation, wired to the DTOs in `ghl-models`.

# API design

Every generated method follows the same predictable shape:

    async fn <name>(&self, <path params…>, params: &XParams, body: &Dto) -> Result<Resp>

- **Path parameters** are positional `&str` arguments in URL order (at most 3).
- **Query parameters** collapse into one generated `XParams` struct — required
  fields are constructor arguments, optional ones are builder methods. The
  argument is omitted entirely for operations with no query parameters.
- **Request bodies** take a reference to the generated DTO from `ghl-models`.
- **Returns** the response DTO the spec names, or `serde_json::Value` when it
  names none (about a quarter of operations).

Modules that already have a hand-written service (contacts, opportunities,
conversations, calendars, locations) get a second `impl` block on that same
service, so `ghl.contacts()` exposes both the curated helpers and the full
generated surface. Generated names that would collide with a hand-written
method get an `_op` suffix.
"""
import json
import keyword
import pathlib
import re
import sys
from collections import defaultdict

METHODS = ("get", "post", "put", "patch", "delete")

# Services that already exist by hand, with the method names they occupy.
HANDWRITTEN = {
    "contacts": ("ContactsService", {"create", "get", "update", "delete", "list"}),
    "opportunities": (
        "OpportunitiesService",
        {"pipelines", "create", "get", "update", "update_status", "delete", "search"},
    ),
    "conversations": ("ConversationsService", {"search", "messages", "send_message"}),
    "calendars": (
        "CalendarsService",
        {"list", "free_slots", "create_appointment", "get_appointment"},
    ),
    "locations": ("LocationsService", {"get", "search"}),
}

RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async",
    "await", "abstract", "become", "box", "do", "final", "macro", "override",
    "priv", "try", "typeof", "unsized", "virtual", "yield", "union",
}

# Noise that makes method names worse, stripped from spec summaries.
SUMMARY_NOISE = re.compile(
    r"^(api to |api |endpoint to |this (api|endpoint) (will |is used to )?)", re.I
)


def snake(name):
    s = re.sub(r"[^0-9a-zA-Z]+", "_", str(name))
    s = re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", s)
    s = re.sub(r"(?<=[A-Z])([A-Z][a-z])", r"_\1", s)
    s = re.sub(r"_+", "_", s).lower().strip("_")
    if not s:
        s = "field"
    if s[0].isdigit():
        s = "n" + s
    if s in RUST_KEYWORDS or keyword.iskeyword(s):
        s += "_"
    return s


def pascal(name):
    parts = [p for p in re.split(r"[^0-9a-zA-Z]+", str(name)) if p]
    out = "".join(p[0].upper() + p[1:] for p in parts)
    if not out:
        out = "Unnamed"
    if out[0].isdigit():
        out = "N" + out
    return out


# rustdoc's `bare_urls` lint fires on the raw links HighLevel puts in its
# descriptions, so make them proper autolinks.
_BARE_URL = re.compile(r"(?<![<(\[])(https?://[^\s<>\)\]]+)")


_HTML_TAG = re.compile(r"</?[a-zA-Z][^>]*>")


def _wrap_bare_urls(text):
    """Strip HighLevel's inline HTML, then autolink what's left.

    Descriptions contain fragments like `<a href="https://…">docs</a>`; leaving
    them in trips rustdoc's `invalid_html_tags`, and wrapping the URL inside the
    attribute makes it worse. Drop the markup first, then autolink bare URLs.
    """
    text = _HTML_TAG.sub(" ", text)
    text = text.replace("&nbsp;", " ").replace("&amp;", "&")
    return _BARE_URL.sub(r"<\1>", text)


def doc(text, indent="    ", limit=600):
    if not text:
        return []
    t = re.sub(r"<br\s*/?>", " ", str(text))
    t = _wrap_bare_urls(re.sub(r"\s+", " ", t).strip()[:limit])
    if not t:
        return []
    out, line = [], ""
    for w in t.split():
        if len(line) + len(w) + 1 > 84:
            out.append(f"{indent}/// {line}")
            line = w
        else:
            line = f"{line} {w}".strip()
    if line:
        out.append(f"{indent}/// {line}")
    return out


def method_name(summary, http, path):
    base = SUMMARY_NOISE.sub("", (summary or "").strip())
    name = snake(base) if base else ""
    if not name or len(name) > 60:
        segs = [s for s in path.strip("/").split("/") if s and not s.startswith("{")]
        name = snake(f"{http}_{'_'.join(segs)}") or f"{http}_root"
    return name


class ServiceGen:
    def __init__(self, module, spec, api_version="v2"):
        self.module = module
        self.api_version = api_version
        self.rust_mod = snake(module)
        self.spec = spec
        self.schemas = (spec.get("components") or {}).get("schemas") or {}
        self.ops = []
        for path, item in (spec.get("paths") or {}).items():
            shared = item.get("parameters") or []
            for http in METHODS:
                op = item.get(http)
                if op:
                    self.ops.append(self._parse(http, path, op, shared))
        self.ops.sort(key=lambda o: (o["path"], o["http"]))
        self._assign_names()

    def _model(self, ref):
        """A $ref to a schema in this spec -> the ghl-models path."""
        name = ref.split("/")[-1]
        if name in self.schemas:
            return f"models::{pascal(name)}"
        return None

    def _parse(self, http, path, op, shared):
        query = []
        seen = set()
        # Spec metadata, keyed by name — used for docs and query params.
        declared = {}
        for p in list(shared) + list(op.get("parameters") or []):
            if "$ref" in p or p.get("in") not in ("path", "query"):
                continue
            key = (p.get("in"), p.get("name"))
            if key in seen:
                continue
            seen.add(key)
            sch = p.get("schema") or {}
            entry = {
                "name": p.get("name"),
                "required": bool(p.get("required")) or p.get("in") == "path",
                "type": sch.get("type", "string"),
                "enum": sch.get("enum"),
                "desc": p.get("description"),
            }
            declared[(p["in"], p["name"])] = entry
            if p["in"] == "query":
                query.append(entry)

        # Path parameters come from the URL template, not the parameter list:
        # some specs omit a placeholder they nonetheless require (e.g. noteId).
        path_params = []
        for placeholder in re.findall(r"\{([^}]+)\}", path):
            meta = declared.get(("path", placeholder))
            path_params.append(meta or {
                "name": placeholder,
                "required": True,
                "type": "string",
                "enum": None,
                "desc": None,
            })

        version = None
        for p in list(op.get("parameters") or []) + list(shared):
            if p.get("name") == "Version":
                sch = p.get("schema") or {}
                vals = sch.get("enum") or ([sch["default"]] if sch.get("default") else [])
                if vals:
                    version = vals[0]

        body_type = None
        bs = (((op.get("requestBody") or {}).get("content") or {})
              .get("application/json") or {}).get("schema") or {}
        if "$ref" in bs:
            body_type = self._model(bs["$ref"]) or "serde_json::Value"
        elif bs or op.get("requestBody"):
            body_type = "serde_json::Value"

        ret = None
        for code, r in (op.get("responses") or {}).items():
            if str(code).startswith("2"):
                sch = (((r.get("content") or {}).get("application/json") or {})
                       .get("schema") or {})
                if "$ref" in sch:
                    ret = self._model(sch["$ref"])
                break

        return {
            "http": http.upper(),
            "path": path,
            "summary": (op.get("summary") or "").strip(),
            "desc": (op.get("description") or "").strip(),
            "path_params": path_params,
            "query": query,
            "body": body_type,
            "ret": ret or "serde_json::Value",
            "version": version,
            "scopes": sorted({s for sec in (op.get("security") or [])
                              for _, sc in sec.items() for s in (sc or [])}),
        }

    def _merges_with_handwritten(self):
        return self.api_version == "v2" and self.module in HANDWRITTEN

    def _assign_names(self):
        taken = set()
        if self._merges_with_handwritten():
            taken |= HANDWRITTEN[self.module][1]
        for op in self.ops:
            base = method_name(op["summary"], op["http"].lower(), op["path"])
            name = base
            if name in taken:
                # A hand-written method owns this name, or two ops share a summary.
                name = f"{base}_op"
                n = 2
                while name in taken:
                    name = f"{base}_op{n}"
                    n += 1
            taken.add(name)
            op["fn"] = name
            op["params_ty"] = pascal(name) + "Params"

    # ---------- rendering ----------

    def _query_type(self, q):
        if q["type"] == "boolean":
            return "bool"
        if q["type"] == "integer":
            return "i64"
        if q["type"] == "number":
            return "f64"
        return "String"

    def render_params_struct(self, op):
        """A params struct for query args: required in `new`, optional as setters."""
        if not op["query"]:
            return ""
        ty = op["params_ty"]
        req = [q for q in op["query"] if q["required"]]
        opt = [q for q in op["query"] if not q["required"]]

        out = []
        out += doc(f"Query parameters for [`{self.svc_name()}::{op['fn']}`].", indent="")
        out.append("#[derive(Debug, Clone, Default)]")
        out.append(f"pub struct {ty} {{")
        for q in op["query"]:
            dls = doc(q["desc"])
            if not dls:
                dls = [f"    /// `{q['name']}` query parameter."]
            out.extend(dls)
            if q["enum"]:
                vals = ", ".join(f"`{v}`" for v in q["enum"][:10] if v is not None)
                if vals:
                    out += doc(f"Allowed values: {vals}.")
            if q["required"]:
                out.append("    /// Required by the API.")
            t = self._query_type(q)
            out.append(f"    pub {snake(q['name'])}: "
                       + (t if q["required"] else f"Option<{t}>")
                       + ",")
        out.append("}")
        out.append("")
        out.append(f"impl {ty} {{")
        # constructor takes the required params
        args = ", ".join(
            f"{snake(q['name'])}: impl Into<{self._query_type(q)}>"
            if self._query_type(q) == "String"
            else f"{snake(q['name'])}: {self._query_type(q)}"
            for q in req
        )
        out.append(f"    /// Start from the parameters the API requires.")
        out.append(f"    pub fn new({args}) -> Self {{")
        out.append("        Self {")
        for q in req:
            f = snake(q["name"])
            if self._query_type(q) == "String":
                out.append(f"            {f}: {f}.into(),")
            else:
                out.append(f"            {f},")   # shorthand — clippy prefers it
        if opt:
            out.append("            ..Default::default()")
        out.append("        }")
        out.append("    }")
        for q in opt:
            t = self._query_type(q)
            setter = snake(q["name"])
            if setter in {"new"}:
                setter += "_"
            out.append("")
            dls = doc(q["desc"])
            if not dls:
                dls = [f"    /// Set the `{q['name']}` query parameter."]
            out.extend(dls)
            if t == "String":
                out.append(f"    pub fn {setter}(mut self, v: impl Into<String>) -> Self {{")
                out.append(f"        self.{snake(q['name'])} = Some(v.into());")
            else:
                out.append(f"    pub fn {setter}(mut self, v: {t}) -> Self {{")
                out.append(f"        self.{snake(q['name'])} = Some(v);")
            out.append("        self")
            out.append("    }")
        out.append("")
        out.append("    fn to_query(&self) -> Vec<(String, String)> {")
        mut_kw = "mut " if opt else ""
        if req:
            out.append(f"        let {mut_kw}q: Vec<(String, String)> = vec![")
            for q in req:
                f = snake(q["name"])
                val = f"self.{f}.clone()" if self._query_type(q) == "String" else f"self.{f}.to_string()"
                out.append(f'            ("{q["name"]}".into(), {val}),')
            out.append("        ];")
        else:
            out.append(f"        let {mut_kw}q: Vec<(String, String)> = Vec::new();")
        for q in opt:
            f = snake(q["name"])
            out.append(f"        if let Some(v) = &self.{f} {{")
            out.append(f'            q.push(("{q["name"]}".into(), v.to_string()));')
            out.append("        }")
        out.append("        q")
        out.append("    }")
        out.append("}")
        out.append("")
        return "\n".join(out)

    def svc_name(self):
        if self._merges_with_handwritten():
            return HANDWRITTEN[self.module][0]
        return pascal(self.module) + "Service"

    def render_method(self, op):
        out = []
        out += doc(op["summary"] or f"{op['http']} {op['path']}")
        if op["desc"] and op["desc"] != op["summary"]:
            out.append("    ///")
            out += doc(op["desc"])
        out.append("    ///")
        out.append(f"    /// `{op['http']} {op['path']}`")
        if op["scopes"]:
            out.append("    ///")
            out.append("    /// Requires scope: " + ", ".join(f"`{s}`" for s in op["scopes"]) + ".")

        args = ["&self"]
        for p in op["path_params"]:
            args.append(f"{snake(p['name'])}: &str")
        if op["query"]:
            args.append(f"params: &{op['params_ty']}")
        if op["body"]:
            args.append(f"body: &{op['body']}")
        out.append(f"    pub async fn {op['fn']}(")
        out.append("        " + ",\n        ".join(args) + ",")
        out.append(f"    ) -> Result<{op['ret']}> {{")

        # path building
        if op["path_params"]:
            fmt = re.sub(r"\{[^}]+\}", "{}", op["path"])
            names = ", ".join(
                f"crate::services::encode({snake(p['name'])})" for p in op["path_params"]
            )
            out.append(f'        let path = format!("{fmt}", {names});')
            path_expr = "&path"
        else:
            path_expr = f'"{op["path"]}"'

        out.append("        let query = "
                   + ("params.to_query();" if op["query"] else "Vec::new();"))
        body_expr = "Some(body)" if op["body"] else "None::<&()>"
        # v3 endpoints need their own Version header; the spec value wins when set.
        fallback = '"v3"' if self.api_version == "v3" else None
        if op["version"]:
            version = f'Some("{op["version"]}")'
        elif fallback:
            version = f"Some({fallback})"
        else:
            version = "None"
        out.append("        self.client")
        out.append("            .send_versioned(")
        out.append(f"                reqwest::Method::{op['http']},")
        out.append(f"                {path_expr},")
        out.append("                &query,")
        out.append(f"                {body_expr},")
        out.append(f"                {version},")
        out.append("            )")
        out.append("            .await")
        out.append("    }")
        return "\n".join(out)

    def render(self):
        mod_name = self.module
        feature = self.module
        v = self.api_version
        if self._merges_with_handwritten():
            access = (f"//! These methods live on the same"
                      f" [`{self.svc_name()}`](crate::{self.rust_mod}::{self.svc_name()}) you get"
                      f" from [`Ghl::{self.rust_mod}`](crate::Ghl::{self.rust_mod}), alongside"
                      f" the hand-written helpers.")
        elif v == "v3":
            access = (f"//! Access via"
                      f" [`Ghl::v3`](crate::Ghl::v3)`().{self.rust_mod}()`. These endpoints send"
                      f" `Version: v3`.")
        else:
            access = f"//! Access via [`Ghl::{self.rust_mod}`](crate::Ghl::{self.rust_mod})."
        header = [
            "// @generated by xtask/generate_services.py — do not edit by hand.",
            f"//! `{mod_name}` — typed methods for all {len(self.ops)} API {v} operations",
            "//! in this module.",
            "//!",
            access,
            "//!",
            f"//! Request and response types come from [`ghl_models::{v}::{self.rust_mod}`]"
            f"(https://docs.rs/ghl-models/latest/ghl_models/{v}/{self.rust_mod}/);"
            f" every endpoint is also documented in the",
            f"//! [`{mod_name}` API reference](https://github.com/Shahroz/ghl-rs/blob/main/docs/api/{mod_name}.md).",
            "//!",
            f'//! Enable with `features = ["{feature}"]`.',
            "",
            "#![allow(clippy::too_many_arguments)]",
            "",
            ("use crate::client::Ghl;" if not self._merges_with_handwritten() else ""),
            "use crate::error::Result;",
            f"use ghl_models::{v}::{self.rust_mod} as models;",
            "",
        ]

        body = []
        if not self._merges_with_handwritten():
            via = (f"[`Ghl::v3`](crate::Ghl::v3)`().{self.rust_mod}()`" if v == "v3"
                   else f"[`Ghl::{self.rust_mod}`](crate::Ghl::{self.rust_mod})")
            body += doc(
                f"Typed access to the `{mod_name}` API {v} surface "
                f"({len(self.ops)} operations). Obtained via {via}.",
                indent="",
            )
            body.append("#[derive(Debug, Clone)]")
            body.append(f"pub struct {self.svc_name()} {{")
            body.append("    pub(crate) client: Ghl,")
            body.append("}")
            body.append("")
            body.append(f"impl {self.svc_name()} {{")
            body.append("    pub(crate) fn new(client: Ghl) -> Self {")
            body.append("        Self { client }")
            body.append("    }")
            body.append("}")
            body.append("")
        else:
            body.append(f"use crate::{self.rust_mod}::{self.svc_name()};")
            body.append("")

        params_structs = [self.render_params_struct(op) for op in self.ops]
        body += [p for p in params_structs if p]

        body.append(f"impl {self.svc_name()} {{")
        for op in self.ops:
            body.append(self.render_method(op))
            body.append("")
        body.append("}")
        return "\n".join(header + body) + "\n"


def main():
    docs_root = pathlib.Path(sys.argv[1])
    sdk = pathlib.Path(sys.argv[2])
    out_dir = sdk / "src/services"
    (out_dir / "v3").mkdir(parents=True, exist_ok=True)

    sets = {"v2": (docs_root / "apps", out_dir), "v3": (docs_root / "apps/v3", out_dir / "v3")}
    gens = {"v2": [], "v3": []}
    for version, (spec_dir, dest) in sets.items():
        for f in sorted(spec_dir.glob("*.json")):
            spec = json.loads(f.read_text())
            if not (spec.get("paths") or {}):
                continue
            g = ServiceGen(f.stem.replace("-v3", ""), spec, api_version=version)
            if not g.ops:
                continue
            (dest / f"{g.rust_mod}.rs").write_text(g.render())
            gens[version].append(g)

    # services/v3/mod.rs — plus the V3 namespace the client hands out
    v3 = sorted(gens["v3"], key=lambda x: x.module)
    lines = [
        "// @generated by xtask/generate_services.py — do not edit by hand.",
        "//! Generated typed services for **GoHighLevel API v3**.",
        "//!",
        "//! Reach these through [`Ghl::v3`](crate::Ghl::v3):",
        "//!",
        "//! ```ignore",
        "//! let out = ghl.v3().contacts().get_duplicate_contact(&params).await?;",
        "//! ```",
        "//!",
        "//! Every call sends `Version: v3`. Modules are behind the same cargo",
        "//! features as their v2 counterparts.",
        "",
        "use crate::client::Ghl;",
        "",
    ]
    for g in v3:
        lines.append(f"/// `{g.module}` — {len(g.ops)} v3 operations.")
        lines.append(f'#[cfg(feature = "{g.module}")]')
        lines.append(f'#[cfg_attr(docsrs, doc(cfg(feature = "{g.module}")))]')
        lines.append(f"pub mod {g.rust_mod};")
    lines += [
        "",
        "/// Entry point for the API v3 services. Obtained via [`Ghl::v3`](crate::Ghl::v3).",
        "#[derive(Debug, Clone)]",
        "pub struct V3 {",
        "    /// Only the feature-gated accessors below read this, so with no module",
        "    /// features enabled it is legitimately unused.",
        "    #[allow(dead_code)]",
        "    pub(crate) client: Ghl,",
        "}",
        "",
        "impl V3 {",
    ]
    for g in v3:
        svc = f"{g.rust_mod}::{pascal(g.module)}Service"
        lines += [
            f"    /// The `{g.module}` API v3 surface ({len(g.ops)} operations).",
            f'    #[cfg(feature = "{g.module}")]',
            f'    #[cfg_attr(docsrs, doc(cfg(feature = "{g.module}")))]',
            f"    pub fn {g.rust_mod}(&self) -> {svc} {{",
            f"        {svc}::new(self.client.clone())",
            "    }",
            "",
        ]
    lines.append("}")
    (out_dir / "v3/mod.rs").write_text("\n".join(lines) + "\n")

    # services/mod.rs
    v2 = sorted(gens["v2"], key=lambda x: x.module)
    lines = [
        "// @generated by xtask/generate_services.py — do not edit by hand.",
        "//! Generated typed services — one per GoHighLevel API module, each",
        "//! covering every operation that module exposes.",
        "//!",
        "//! This module holds the **API v2** services (the stable API), reached",
        "//! directly on the client: `ghl.invoices()`. API [`v3`] services live in the",
        "//! submodule and are reached via `ghl.v3().invoices()`.",
        "//!",
        "//! Each module here is behind a cargo feature of the same name, which also",
        "//! pulls in the matching `ghl-models` DTOs. Enable only what you use:",
        "//! compiling every module is far slower than compiling one.",
        "",
        "pub mod v3;",
        "",
        "/// Percent-encode a value being interpolated into a URL path segment.",
        "///",
        "/// Only the generated services call this, and each is feature-gated, so with",
        "/// no module features enabled it is legitimately unused.",
        "#[allow(dead_code)]",
        "pub(crate) fn encode(s: &str) -> String {",
        "    s.chars()",
        "        .flat_map(|c| match c {",
        "            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => vec![c],",
        "            _ => format!(\"%{:02X}\", c as u32).chars().collect(),",
        "        })",
        "        .collect()",
        "}",
        "",
    ]
    for g in v2:
        lines.append(f"/// `{g.module}` — {len(g.ops)} v2 operations.")
        lines.append(f'#[cfg(feature = "{g.module}")]')
        lines.append(f'#[cfg_attr(docsrs, doc(cfg(feature = "{g.module}")))]')
        lines.append(f"pub mod {g.rust_mod};")
    (out_dir / "mod.rs").write_text("\n".join(lines) + "\n")

    for version in ("v2", "v3"):
        total = sum(len(g.ops) for g in gens[version])
        print(f"{version}: {total} methods across {len(gens[version])} modules")
    return gens


if __name__ == "__main__":
    main()
