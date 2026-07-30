# Homebrew formula for ghl-mcp.
#
# Install without cloning:
#   brew tap shahroz/ghl-rs https://github.com/Shahroz/ghl-rs
#   brew install ghl-mcp
#
# Using the main repo as the tap avoids a second homebrew-* repo; `brew tap`
# accepts an explicit URL for exactly this case. The version and checksums below
# are bumped by .github/workflows/release.yml on each tag.
class GhlMcp < Formula
  desc "MCP server for GoHighLevel CRM - typed, agency-grade, single static binary"
  homepage "https://github.com/Shahroz/ghl-rs"
  version "0.5.1"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/Shahroz/ghl-rs/releases/download/v0.5.1/ghl-mcp-aarch64-apple-darwin.tar.gz"
      sha256 "ba7a3cab722e03707f138706c3625042fe7c491a46dd294736e6e39994d90d33"
    end
    on_intel do
      url "https://github.com/Shahroz/ghl-rs/releases/download/v0.5.1/ghl-mcp-x86_64-apple-darwin.tar.gz"
      sha256 "ec6ee793896dd30640f83abb266b0d4ab45dd077e6f1b24e313da3dfe527600b"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Shahroz/ghl-rs/releases/download/v0.5.1/ghl-mcp-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "b7b000ab333d62eb0b1e4dcdac037340fd1a5a2a481c28d239a8f142c142bbe4"
    end
    on_intel do
      url "https://github.com/Shahroz/ghl-rs/releases/download/v0.5.1/ghl-mcp-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "d300ec6ca71304de9ff48d3a804cb0bb964c258e31802ffe66563929b1e777dc"
    end
  end

  def install
    bin.install "ghl-mcp"
  end

  def caveats
    <<~EOS
      ghl-mcp needs GoHighLevel credentials. Create a Private Integration Token
      in your sub-account (Settings -> Private Integrations), then either:

        export GHL_PIT_TOKEN=pit-...
        export GHL_LOCATION_ID=...

      or point your MCP host at the binary with those set in its env block. See
      https://github.com/Shahroz/ghl-rs#quickstart--mcp-server
    EOS
  end

  test do
    assert_match "ghl-mcp #{version}", shell_output("#{bin}/ghl-mcp --version")
    # Without credentials the server must refuse to start, with a clear message.
    output = shell_output("#{bin}/ghl-mcp 2>&1", 1)
    assert_match "no credentials", output
  end
end
