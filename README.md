# Roamari

Roamari is the Astrohacker TermSurf browser client: Chromium in a pane.

## Install

Apple silicon, macOS 26 (Tahoe) or newer:

```nu
brew trust astrohackerlabs/astrohacker
brew tap astrohackerlabs/astrohacker
brew install roamari
```

Browsing requires [Astrohacker TermSurf](https://github.com/astrohackerlabs/termsurf), installed separately. Run `roamari` inside a TermSurf pane. The Chromium engine remains `ah-chromiumd` from TermSurf.

```nu
roamari --version
roamari https://example.com
roamari --browser chromium https://example.com
```

`--version` and `--help` work outside TermSurf. Opening a page does not.

## Source

This repository is the public source for the `roamari` crate. Product development happens in the private Astrohacker monorepo. Do not expect this tree to include TermSurf, Chromium, or Homebrew publisher scripts.
