#!/usr/bin/env node
// Downloads the prebuilt ghl-mcp binary for this platform from GitHub Releases.
// The published binary is pure Rust; Node is only the delivery mechanism, so
// that `npx ghl-mcp` works in MCP host configs.
const fs = require("fs");
const path = require("path");
const https = require("https");
const { version } = require("./package.json");

const TARGETS = {
  "darwin-arm64": "aarch64-apple-darwin",
  "darwin-x64": "x86_64-apple-darwin",
  "linux-arm64": "aarch64-unknown-linux-gnu",
  "linux-x64": "x86_64-unknown-linux-gnu",
  "win32-x64": "x86_64-pc-windows-msvc",
};

const key = `${process.platform}-${process.arch}`;
const target = TARGETS[key];
if (!target) {
  console.error(
    `ghl-mcp: no prebuilt binary for ${key}.\n` +
      `Install from source instead:  cargo install ghl-mcp`
  );
  process.exit(1);
}

const ext = process.platform === "win32" ? ".exe" : "";
const archive = `ghl-mcp-${target}.tar.gz`;
const url = `https://github.com/Shahroz/ghl-rs/releases/download/v${version}/${archive}`;
const binDir = path.join(__dirname, "bin");
fs.mkdirSync(binDir, { recursive: true });
const out = path.join(binDir, `ghl-mcp${ext}`);

function get(u, redirects = 0) {
  if (redirects > 5) throw new Error("too many redirects");
  https.get(u, { headers: { "User-Agent": "ghl-mcp-installer" } }, (res) => {
    if (res.statusCode >= 300 && res.statusCode < 400 && res.headers.location) {
      return get(res.headers.location, redirects + 1);
    }
    if (res.statusCode !== 200) {
      console.error(
        `ghl-mcp: download failed (HTTP ${res.statusCode}) from ${u}\n` +
          `Fall back to:  cargo install ghl-mcp`
      );
      process.exit(1);
    }
    const tmp = `${out}.tar.gz`;
    const file = fs.createWriteStream(tmp);
    res.pipe(file);
    file.on("finish", () => {
      file.close(() => {
        require("child_process").execFileSync("tar", ["xzf", tmp, "-C", binDir]);
        fs.unlinkSync(tmp);
        if (process.platform !== "win32") fs.chmodSync(out, 0o755);
        console.log(`ghl-mcp ${version} installed for ${key}`);
      });
    });
  }).on("error", (e) => {
    console.error(`ghl-mcp: ${e.message}\nFall back to:  cargo install ghl-mcp`);
    process.exit(1);
  });
}
get(url);
