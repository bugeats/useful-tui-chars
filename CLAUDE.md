# useful-tui-chars

Rust program that prints TUI-relevant Unicode characters in aligned markdown tables: glyph, Rust escape, official Unicode name.

Character names from `unicode_names2` (Unicode Character Database). Display widths from `unicode-width` for terminal-accurate column alignment.

## Architecture

- `flake.nix` — crane-based Nix build. `nix run` produces the table. `packages.readme` generates README content.
- `publish.sh` — builds the `readme` derivation and copies result to `README.md`.
- `src/main.rs` — `RANGES` constant (14 Unicode blocks) and entry point. Passes ranges to `Document::new`.
- `src/document.rs` — `Document` type. Constructed from range tuples, `Display` renders full markdown (title, provenance, TOC, sections).
- `src/block.rs` — `BlockRange` (`From` tuple) and `CharEntry` (`From` char). `Display for BlockRange` renders one section table.

## Current Focus

Feature-complete. `./publish.sh` regenerates README.md from the `readme` flake output. Potential next: filter to useful subset, CI-driven readme regeneration.
