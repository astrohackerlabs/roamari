# Roamari

Roamari is the Astrohacker TermSurf browser client: Chromium in a pane.

## Install

Apple silicon, macOS 26 (Tahoe) or newer:

```nu
brew trust astrohackerlabs/astrohacker
brew tap astrohackerlabs/astrohacker
brew install roamari
```

Browsing requires a TermSurf-protocol host. [Astrohacker TermSurf](https://github.com/astrohackerlabs/termsurf) is one such host, installed separately. Run `roamari` inside a TermSurf pane. This formula ships `roamari-chromiumd` and the Chromium payload.

```nu
roamari --version
roamari-chromiumd --version
roamari https://example.com
roamari --browser chromium https://example.com
```

`--version` and `--help` work outside TermSurf. Opening a page does not.

## Source

This repository is the public source for the `roamari` and `roamari-chromiumd` crates. Product development happens in the private Astrohacker monorepo. Do not expect this tree to include TermSurf, a Chromium checkout, or Homebrew publisher scripts.
