# chars

A simple rust program that generates a list of useful ASCII and ANSI box drawing characters for the purposed of building a TUI.

Output should be a in table column format. Example:

```
├    \u{251C}     Light vertical and right
┝    \u{251D}     Vertical light and right heavy
┞    \u{251E}     Up heavy and right down light  
```

Where the middle column is the Rust string escape format.

## Current Focus

Init a new Rust flake using cranelib. `nix run` should generate the list.
