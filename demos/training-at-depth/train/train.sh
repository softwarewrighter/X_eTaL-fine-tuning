#!/usr/bin/env bash
# The offline runs of experiment D4: the plain skip, HC and mHC at 12,
# 24 and 48 layers, 800 steps each, from the same start, by the same
# program the page runs (training-at-depth.xtl) with its settings
# changed: the scheme (S, sink), the depth (L), the steps, and no
# gradient check (the page's run checks it). Each run's record (a row a
# step: loss, gradient size, forward gain, backward gain, proxy, NaN or
# infinite) goes to data/SCHEME-L.txt, and data/runs.txt says what made
# them. Nothing is random: a rerun gives the same files.
#   demos/training-at-depth/train/train.sh [SCHEME-L ...]   (default: all nine)
set -euo pipefail
here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
root="$(cd "$here/../.." && pwd)"
xetal="$("$root/scripts/xetal-ml.sh")"
export XETAL_PATH="$("$root/scripts/libpath.sh")"
steps=800
runs=("$@")
[ ${#runs[@]} -gt 0 ] || runs=(baseline-12 hc-12 mhc-12 baseline-24 hc-24 mhc-24 baseline-48 hc-48 mhc-48)
work="$(mktemp -d)"; trap 'rm -rf "$work"' EXIT
one() {
  local run="$1" scheme="${1%-*}" depth="${1##*-}" s sink
  case "$scheme" in
    baseline) s=1; sink=1.0 ;;
    hc) s=4; sink=0.0 ;;
    mhc) s=4; sink=1.0 ;;
    *) echo "train: no scheme $scheme" >&2; return 1 ;;
  esac
  sed -e "s/^S := 4$/S := $s/" -e "s/^sink := 1.0$/sink := $sink/" -e "s/^L := 12$/L := $depth/" \
      -e "s/^steps := 50$/steps := $steps/" -e "s/^checks := 9$/checks := 0/" "$here/training-at-depth.xtl" > "$work/$run.xtl"
  local start=$SECONDS
  (cd "$work" && "$xetal" run --seed 1 "$run.xtl") | sed -n 1p > "$here/data/$run.txt"
  echo "train: $run, $steps steps, $((SECONDS - start)) s"
}
pids=()
for run in "${runs[@]}"; do one "$run" & pids+=($!); while [ "$(jobs -rp | wc -l)" -ge 3 ]; do sleep 1; done; done
for p in "${pids[@]}"; do wait "$p"; done
{
  echo "# The offline runs of training-at-depth: train/train.sh, from training-at-depth.xtl."
  echo "steps = $steps"
  echo "xetal_ml = $(cat "$root/XETAL_ML_COMMIT")"
  echo "xetal_libraries = $(cat "$root/XETAL_LIBRARIES_COMMIT")"
  echo "program = $(git -C "$root" hash-object "$here/training-at-depth.xtl")"
} > "$here/data/runs.txt"
