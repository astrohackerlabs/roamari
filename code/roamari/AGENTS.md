# Roamari

TermSurf browser client: Chromium in a pane. The public command is `roamari`.
Rust lives in `rs/`. The former crate was `ahweb` at `code/termsurf/rs/ahweb`;
do not revive that name or path.

Keep `ah-chromiumd` as a separately named engine. Roamari compiles
`code/termsurf/proto/termsurf.proto` and talks to the host compositor, then to
the engine socket from `BrowserReady`. Do not merge client and engine into one
binary. Do not add an `ahweb` PATH shim.

Version independently of TermSurf. The first Homebrew formula defaults to
0.1.0. TermSurf must not rewrite this crate's version. TermSurf may still
stage the binary until a later experiment adds cask `depends_on` and stops
bundling. Publish with `scripts/release-roamari.nu` onto
`astrohackerlabs/astrohacker` `Formula/roamari.rb`. Ryan types the publication
phrase. Do not add a roamari cask or edit `Casks/termsurf.rb` here.

Build and identity:

```nu
nu scripts/build.nu roamari --release
cargo test --locked -p roamari
target/release/roamari --version
```

Product smoke must run inside Astrohacker TermSurf and pass the Chromium engine
with `--browser`, following the root two-shell rules.

Use the root issues-and-experiments workflow and ah. Closed records may still
say `ahweb`; do not rewrite them.
