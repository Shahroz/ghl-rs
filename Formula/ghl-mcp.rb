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
  version "0.5.0"
  license any_of: ["MIT", "Apache-2.0"]

  on_macos do
    on_arm do
      url "https://github.com/Shahroz/ghl-rs/releases/download/v0.5.0/ghl-mcp-aarch64-apple-darwin.tar.gz"
      sha256 "64516574da3797d98399a84176688db3a446c29aab3f1963e9eb95d1ad2bb931"
    end
    on_intel do
      url "https://github.com/Shahroz/ghl-rs/releases/download/v0.5.0/ghl-mcp-x86_64-apple-darwin.tar.gz"
      sha256 "aab3f6bd5b94d4d7a269d261ca64b60f0efc222f6c4604cda65c40f200b6afad"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/Shahroz/ghl-rs/releases/download/v0.5.0/ghl-mcp-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "d6aa09179afcc3396c63abbb42b6e7137520868ef733e75740327637019e5f20"
    end
    on_intel do
      url "https://github.com/Shahroz/ghl-rs/releases/download/v0.5.0/ghl-mcp-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "746769346ca81dc1fa56970da22e7e2b6f4e4feafcd6fe679f11aef59ecac571"
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
