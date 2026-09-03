# AGENTS.md — Spatial Audio Server (Antopia)

## Project

Museum Victoria's open-source **spatial audio server** (`museumsvictoria/spatial_audio_server`),
a nannou 0.5 / conrod 0.59 GUI application, locally customised ("Antopia") by Richard Pilkington.

- Platform: Intel macOS 12.7.6
- Build system: pure Cargo (no CMake) — workspace-less single crate `audio_server`
- Upstream: https://github.com/museumsvictoria/spatial_audio_server
- Local work happens on the `antopia` branch; `master` tracks upstream.

## Toolchain (critical)

The code is 2018–2020 vintage and will not build on the newest Rust
(Homebrew rustc 1.90 fails); the **1.54.0** rustup toolchain is verified.

- `rust-toolchain` file pins **1.54.0** (applies to the rustup shims).
- PATH ordering matters: `~/.cargo/bin` (rustup shims, 1.54.0) must precede
  `/usr/local/bin` (Homebrew cargo 1.90, which ignores the pin). Note the
  default PATH can be broken in some shells — use the full override below.
- `coreaudio-sys` is vendored at `vendor/coreaudio-sys` via `[patch.crates-io]`
  (pre-generated bindings; see `vendor/coreaudio-sys/build.rs`).
- Always build with:

```sh
PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" cargo build --release
PATH="$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin" cargo run --release
```

## Vendored crate patches (critical)

Several old dependencies use Rust UB that rustc 1.54 rejects at runtime
(`mem::uninitialized` panics; a null-reference trick in glium's
`implement_vertex!` now compiles to a hard trap). Patched copies live in
`vendor/`:

- `vendor/coreaudio-sys` — pre-generated bindings (bindgen 0.32 can't run on
  this machine's modern libclang).
- `vendor/crossbeam` (0.3.2) — MsQueue/SegQueue sentinel UB removed.
- `vendor/linked-hash-map` (0.5.2) — uninit guard-node UB removed.
- `vendor/glium` (0.21.0) — null-reference in `implement_vertex!` replaced
  (was: instant SIGILL when the GUI drew its first frame).
- `vendor/nannou` (0.5.2) — `inner_size_points` MULTIPLIED pixels by the hidpi
  factor (points = pixels / hidpi). On Retina this laid the UI out at 4x the
  window size: no side menu, broken pan/zoom until a resize event fixed it.

**Gotcha:** `.cargo/config` replaces crates.io with a local directory source,
so cargo IGNORES the `[patch.crates-io]` section. What actually gets compiled
is the copy in `~/.cargo/registry/src/github.com-1ecc6299db9ec823/`. After
editing anything in `vendor/` (or if fixes mysteriously disappear), run:

```sh
sh scripts/sync-vendor.sh
```

then rebuild. Bindings/patches must never be re-generated with bindgen > 0.69
(0.70+ needs Rust 1.77).

## Code layout

- `src/bin/main.rs` — entry point
- `src/lib/audio/` — audio engine, playback, output streams
- `src/lib/soundscape/` — sound spawning, noise-walk, spatialisation (DBAP/panning)
- `src/lib/gui/` — conrod UI
- `src/lib/osc/` — OSC control interface
- `src/lib/project/` — project load/save
- `src/lib/config.rs`, `master.rs`, `installation.rs`, `camera.rs` — shared modules

## Assets & conventions

- `assets/config.json` — global config. **Never edit carelessly**; ask first.
- `assets/projects/<name>/{config,state}.json` — per-project setup and runtime state.
- WAV files in `assets/audio/` must be 48 kHz, 16- or 32-bit.
- Floorplan image: `assets/images/floorplan.png` (BPfloorplan.png is the Antopia one).

## Rules

- **Never `git commit` without asking.** All work is on the `antopia` branch.
- Runtime/audio/GUI testing is done by the user (the agent cannot hear output).
- After any code change: build with the pinned toolchain, then ask the user to test.
- Prefer small, revertable changes; one verified change per commit.
- Packaging uses `nannou-package` (in `~/.cargo/bin`), same PATH override as above.
