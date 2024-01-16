This project implements [Conway's Game of
Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) for the [Einstein
Hat](https://cs.uwaterloo.ca/~csk/hat/) and the
[Spectre](https://cs.uwaterloo.ca/~csk/spectre/).

It's playable at https://hylo.ink/monotile/game.
Project documentation at https://hylo.ink/monotile/docs.

# Instructions
- Left mouse button to draw (change stroke width for a wider stroke)
- Right mouse button to move the camera
- Scroll to zoom
- **Play/Pause** toggles running, **Step** advances by a single generation
- **Update interval** changes how fast generations update
- **Birth**: Iff a dead cell has $`x ∈ \text{Birth}`$ neighbors, it will be alive in the next generation. Multiple (or 0) values possible.
- **Survival**: Iff an alive cell has $`x ∈ \text{Survival}`$ neighbors, it will be alive in the next generation. Multiple (or 0) values possible.
- Standard *Game of Life* rules are set in the UI on startup (**Birth: 3 / Survival: 23**, or **B3/S23** for short)
- **Add/Remove noise** sets some random cells alive/dead


# Fun rules to try

- **B: 3 / S: 234** 
  - forms resilient hexagons; add 5 and/or 6 to Survival for more stability
- **B: 356 / S: 0123456** and variations
  - **Fill** all cells, then **Step** through the generations (or set **Update interval** to 2.0s and press **Play**). You'll see intricate patterns emerging, which change at every turn and slowly devolve into chaos. 
- B: 2 / S: 3
  - fluctuating swarms
- B: 2 / S: 34
- B: 24 / S: 
- B: 34 / S: 124
- B: 2 / S: 2345

# Run
Visit https://hylo.ink/monotile/game (wait a moment until it's loaded, it's a 26MB .wasm file) or

1. [Install Rust](https://www.rust-lang.org/tools/install). Preferably the `nightly` toolchain.
2.
    ```sh
    cargo run --release
    ```
    
    Tested on Linux with Wayland. On native, we are generating a tree which is one level deeper, around 600.000 shapes/cells, whereas on WASM it's around 60.000. Native runs faster, but otherwise there shouldn't be any difference to the WASM build.
