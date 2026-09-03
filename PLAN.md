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
2. **Known GUI issues (from first stable run, 4 Sep 2026)** — investigate:
   - Background cannot be panned/moved; zoom limited (~1 click).
   - Mouse interaction with the floorplan appears non-functional.
   - The side menu with project options is not viewable/accessible (may be a
     window-size/layout issue — check conrod widget placement vs screen size).
3. Soundscape work: identify the first behaviour change desired (spawning,
   noise-walk, or spatialisation), design in plan mode, implement.
4. Packaging via `nannou-package` when a build is ready to deploy.
