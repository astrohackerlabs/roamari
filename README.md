# Roamari

Roamari is the Astrohacker TermSurf browser client: Chromium in a pane.

## Install

Apple silicon, macOS 26 (Tahoe) or newer:

```nu
brew trust astrohackerlabs/astrohacker
brew tap astrohackerlabs/astrohacker
brew uninstall roamari
brew install --cask roamari
```

Browsing requires a TermSurf-protocol host. [Astrohacker TermSurf](https://github.com/astrohackerlabs/termsurf) is one such host, installed separately. Run `roamari` inside a TermSurf pane. This cask ships `roamari` on PATH and the Chromium tree at `/opt/homebrew/opt/roamari-chromiumd/`.

```nu
roamari --version
/opt/homebrew/opt/roamari-chromiumd/roamari-chromiumd --version
roamari https://example.com
roamari --browser /opt/homebrew/opt/roamari-chromiumd/roamari-chromiumd https://example.com
```

`--version` and `--help` work outside TermSurf. Opening a page does not.

## Source

This repository is the public source for the `roamari` and `roamari-chromiumd` crates. Product development happens in the private Astrohacker monorepo. Do not expect this tree to include TermSurf, a Chromium checkout, or Homebrew publisher scripts.
