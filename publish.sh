#!/usr/bin/env bash
set -euo pipefail

nix build .#readme --print-build-logs
cp --force result README.md
