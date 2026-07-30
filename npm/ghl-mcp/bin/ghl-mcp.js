#!/usr/bin/env node
// Thin launcher: exec the real binary, forwarding argv, stdio and exit code.
// stdio must pass through untouched — it carries the MCP protocol.
const path = require("path");
const { spawnSync } = require("child_process");
const bin = path.join(__dirname, process.platform === "win32" ? "ghl-mcp.exe" : "ghl-mcp");
const r = spawnSync(bin, process.argv.slice(2), { stdio: "inherit" });
process.exit(r.status === null ? 1 : r.status);
