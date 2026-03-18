# chars

Rust program that prints TUI-relevant Unicode characters in aligned markdown tables: glyph, Rust escape, official Unicode name.

Character names from `unicode_names2` (Unicode Character Database). Display widths from `unicode-width` for terminal-accurate column alignment.

## Architecture

- `flake.nix` — crane-based Nix build. `nix run` produces the table.
- `src/main.rs` — `BlockRange` and `CharEntry` types. Construction pre-computes entries and column widths; `Display for BlockRange` renders each section.
- 14 blocks in `RANGES` (code-point order): Greek/Coptic, Letterlike Symbols, Arrows, Math Operators, Misc Technical, Enclosed Alphanumerics, Box Drawing, Block Elements, Geometric Shapes, Misc Symbols, Dingbats, Braille, Misc Symbols & Arrows, Legacy Computing.
- Output is a full markdown document: `# Useful TUI Characters`, provenance note, TOC with anchor links, then `## Block (count)` sections.

## Current Focus

Feature-complete as a reference document. Potential next: filter to useful subset, pipe to file.
