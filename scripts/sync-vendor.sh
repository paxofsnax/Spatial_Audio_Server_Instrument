#!/bin/sh
# Sync the patched vendored crates into the local registry source directory.
#
# WHY THIS EXISTS: `.cargo/config` replaces crates.io with a directory source
# (~/.cargo/registry/src/github.com-1ecc6299db9ec823). Cargo IGNORES
# [patch.crates-io] when the source is replaced, so the vendor/ copies are NOT
# what gets compiled — the directory-source copies are. The vendor/ directory is
# the version-controlled record of the patches; this script makes them active.
#
# Run this after:
#   - editing anything in vendor/
#   - cargo re-extracting a crate over the registry copy (e.g. after registry
#     maintenance), which would silently restore unpatched upstream sources.
#
# Usage: sh scripts/sync-vendor.sh   (then rebuild)

set -e
cd "$(dirname "$0")/.."

REG="$HOME/.cargo/registry/src/github.com-1ecc6299db9ec823"

# vendor-dir-name  registry-dir-name
PAIRS="
coreaudio-sys:coreaudio-sys-0.2.2
crossbeam:crossbeam-0.3.2
linked-hash-map:linked-hash-map-0.5.2
glium:glium-0.21.0
"

for pair in $PAIRS; do
    vend="vendor/${pair%%:*}"
    dest="$REG/${pair##*:}"
    if [ -d "$vend" ]; then
        rsync -a --exclude .cargo-checksum.json "$vend/" "$dest/"
        echo "synced $vend -> $dest"
    fi
done

echo "done. Now rebuild: PATH=\"\$HOME/.cargo/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin\" cargo build --release"
