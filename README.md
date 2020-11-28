# cg_rogue

This is a prototype for a card based rogue-like game. The main mechanic is all actions are represented by cards, and hand management is crucial to surviving and succeeding. Very early stages of prototyping.

## Implemented

* Hex based grid
* Movement
* Attacking
* Health
* Initial card UI

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```
