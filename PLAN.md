# Antopia Project Plan

Recovered from opencode session history (30 Aug – 3 Sep 2026) so every future
session has durable context. Source sessions: "Setting up opencode for Rust
project work" and "Rust spatial audio server plan".

## Goal

Continue local development of the Antopia fork of Museum Victoria's
`spatial_audio_server` (nannou 0.5 / conrod 0.59 GUI spatial audio server) on
Intel macOS 12.7.6 — primarily **soundscape behaviour work** in
`src/lib/soundscape/` (sound spawning, noise-walk, DBAP/panning) and
`src/lib/audio/`.

## Setup plan — status

1. **Git baseline** — DONE. Branch `antopia` created from `master`; existing
   Antopia customisations committed as baseline `6143c3c`.
2. **Pin the toolchain** — DONE (adjusted). `rust-toolchain` file pins
   **1.54.0** (the build compiles clean under 1.54; the originally planned
   1.35.0 pin was superseded during setup). Homebrew rustc 1.90 must NOT be
   used — see build command in AGENTS.md.
3. **AGENTS.md** — DONE. Project rules, layout, build/run commands.
4. **Verify build baseline** — DONE (3 Sep 2026). `cargo build --release`
   passes end-to-end. This required vendoring `coreaudio-sys` 0.2.2 with
   pre-generated bindings (upstream's bindgen 0.32 cannot run against this
   machine's modern libclang) — see `vendor/coreaudio-sys/build.rs` and
   commit `c13cc48`.
5. **Workflow for soundscape work** — ACTIVE. Launch opencode from the project
   folder, plan/design first, implement, build with pinned toolchain, user
   tests audio in the GUI, commit after each verified change.

## Environment notes (hard-won, don't rediscover)

- Build command: `PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" cargo build --release`
  (PATH ordering matters: rustup shims before Homebrew cargo).
- `bindgen` 0.69 CLI installed at `~/.cargo/bin/bindgen` (0.70+ emits
  `offset_of!`, needs Rust 1.77 — do not upgrade past 0.69).
- Umbrella header for regenerating CoreAudio bindings: `/tmp/coreaudio-umbrella.h`
  (regenerate against `/Library/Developer/CommandLineTools/SDKs/MacOSX11.3.sdk`,
  flags `--with-derive-default`).
- `Cargo.lock` is gitignored in this repo; it still lives on disk for builds.
- `audio_server` at repo root is the old 2019 binary, not a directory.

## Next steps

1. Runtime smoke test — DONE (4 Sep 2026): GUI runs stable. Fixes required:
   vendored `crossbeam`/`linked-hash-map` UB fixes (runtime panics on rustc
   1.54) and a vendored `glium` fix (null-reference in `implement_vertex!`
   trapped as SIGILL on first frame draw). Ngon movement hardened against
   degenerate `vertices`/`nth` values from saved project state.
2. **GUI issues (first stable run, 4 Sep 2026) — FIXED.** Root cause: nannou
   0.5's `inner_size_points` multiplied pixels by the hidpi factor, so on
   Retina the conrod UI was laid out at 4x the window size (no side menu,
   broken pan/zoom until a resize event fixed it). Fixed in
   `vendor/nannou/src/window.rs` (points = pixels / hidpi). Window size also
   set to 1440x900 in configs to fit the 1440x900-point Retina screen.
3. **Soundscape work — first feature DONE (8 Sep 2026):** **NoiseWalk**
   generative movement (third option alongside Agent/Ngon; velocity-field
   Perlin walk with speed / wobble_scale / normalised_dimensions /
   directional). Local commit `30e24e3`, fork commit `6bb2f71`. Remaining
   directions: spawning/density dynamics, DBAP spatialisation behaviour,
   further movement types.
4. **Packaging** via `nannou-package` when a build is ready to deploy.
5. **Fork assets strategy** — deferred by design: the fork
   (`paxofsnax/Spatial_Audio_Server_Instrument`, branch `antopia`) carries
   code only; upstream assets remain as upstream left them. Decide later how
   to share custom assets (private assets branch, separate repo, or keep
   local).
6. **nannou 0.13 port** — future workstream. The fork's `master` is ~20
   commits ahead of the Antopia base (nannou 0.8→0.10→0.13 migration, ASIO,
   device selection, upstream ngon fix). Merging is NOT viable — porting
   means upgrading the code, swapping the vendored UB patches for modern
   crates and re-verifying.

## GitHub fork (workflow)

- Remote `fork` = https://github.com/paxofsnax/Spatial_Audio_Server_Instrument
- Fork `master` = upstream snapshot (nannou 0.13) — **left untouched**.
- Fork branch `antopia` = Antopia code history, built by cherry-picking
  code-only commits onto the local base (`d129924`), then
  `git push fork antopia-push:antopia`. Push is run by the user in their own
  terminal (credentials never pass through the agent).
- Local branch `antopia` = the real working copy, a superset that also
  carries the custom assets (audio, floorplan, project state). The two
  branches intentionally diverge only under `assets/`.
- http.postBuffer/http.version config fixes applied (RPC 400 on push).
