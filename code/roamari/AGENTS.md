# Roamari

TermSurf-protocol browser: Chromium in a pane. Public commands are `roamari`
and `roamari-chromiumd`. Rust lives in `rs/roamari` (TUI) and
`rs/roamari-chromiumd` (engine). The former TUI crate was `ahweb`; the former
engine crate was `ah-chromiumd`. Do not revive those names or paths.

Roamari compiles `code/termsurf/proto/termsurf.proto` and talks to the host
compositor, then to the engine socket from `BrowserReady`. Do not merge client
and engine into one binary. Do not add an `ahweb` or `ah-chromiumd` PATH shim.

Version independently of TermSurf. Stamp both crates together. The first
Homebrew formula defaults to 0.1.0. TermSurf must not rewrite these crate
versions. TermSurf may still stage the TUI and renamed helper until a later
experiment adds cask `depends_on` and stops bundling. Publish with
`scripts/release-roamari.nu` onto `astrohackerlabs/astrohacker`
`Formula/roamari.rb`. Ryan types the publication phrase. Do not add a roamari
cask or a third engine formula. Do not edit `Casks/termsurf.rb` `depends_on`
here.

Build and identity:

```nu
nu scripts/build.nu roamari --release
nu scripts/build.nu roamari-chromiumd --release
cargo test --locked -p roamari
target/release/roamari --version
target/release/roamari-chromiumd --version
```

Product smoke must run inside a TermSurf-protocol host and pass the Chromium
engine with `--browser`, following the root two-shell rules.

Use the root issues-and-experiments workflow and ah. Closed records may still
say `ahweb` or `ah-chromiumd`; do not rewrite them.
