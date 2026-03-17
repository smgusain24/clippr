cask "clippr" do
  version :latest
  sha256 :no_check

  url "https://github.com/smgusain24/clippr/releases/latest/download/Clippr_universal.dmg"
  name "Clippr"
  desc "Lightweight clipboard manager for macOS"
  homepage "https://github.com/smgusain24/clippr"

  app "Clippr.app"

  zap trash: [
    "~/Library/Application Support/com.clippr.dev",
  ]
end
