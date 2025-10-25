class MagnetoSerge < Formula
  desc "High-performance HTTP/WebSocket proxy for testing with record/replay"
  homepage "https://github.com/taciclei/magneto-serge"
  version "0.2.0"
  license "MIT OR Apache-2.0"

  # Binary releases for different platforms
  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/taciclei/magneto-serge/releases/download/v0.2.0/magneto-serge-v0.2.0-aarch64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_ARM64_SHA256"
    else
      url "https://github.com/taciclei/magneto-serge/releases/download/v0.2.0/magneto-serge-v0.2.0-x86_64-apple-darwin.tar.gz"
      sha256 "PLACEHOLDER_X86_64_SHA256"
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url "https://github.com/taciclei/magneto-serge/releases/download/v0.2.0/magneto-serge-v0.2.0-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_LINUX_ARM64_SHA256"
    else
      url "https://github.com/taciclei/magneto-serge/releases/download/v0.2.0/magneto-serge-v0.2.0-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "PLACEHOLDER_LINUX_X86_64_SHA256"
    end
  end

  def install
    bin.install "magneto"

    # Install CA certificate template
    (pkgshare/"ca").mkpath
    if File.exist?("magneto-ca.pem")
      (pkgshare/"ca").install "magneto-ca.pem"
    end

    # Install documentation
    doc.install "README.md" if File.exist?("README.md")
    doc.install "CHANGELOG.md" if File.exist?("CHANGELOG.md")
    doc.install "LICENSE-MIT" if File.exist?("LICENSE-MIT")
    doc.install "LICENSE-APACHE" if File.exist?("LICENSE-APACHE")
  end

  def caveats
    <<~EOS
      Magneto-Serge uses MITM (Man-in-the-Middle) to intercept HTTPS traffic.

      To use HTTPS interception, you need to trust the generated CA certificate:

      1. Generate the CA certificate (on first run):
         magneto init

      2. Trust the certificate on macOS:
         security add-trusted-cert -d -r trustRoot \\
           -k ~/Library/Keychains/login.keychain \\
           ~/.magneto/ca/magneto-ca.pem

      3. Or on Linux:
         sudo cp ~/.magneto/ca/magneto-ca.pem /usr/local/share/ca-certificates/
         sudo update-ca-certificates

      For more information, visit: #{homepage}
    EOS
  end

  test do
    # Test that the binary runs
    assert_match "Magnéto-Serge", shell_output("#{bin}/magneto --version")

    # Test list command (should work even without cassettes)
    system bin/"magneto", "list", "--cassette-dir", testpath/"cassettes"
  end
end
