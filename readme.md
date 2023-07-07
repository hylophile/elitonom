This project implements [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) for the [Einstein hat](https://cs.uwaterloo.ca/~csk/hat/) (and soon, the [Spectre](https://cs.uwaterloo.ca/~csk/spectre/) as well).

It's hosted and playable at https://hylo.ink/monotile.

# Instructions
- **Play/Pause** toggles running
- Left mouse button to draw (change stroke width for a wider stroke)
- Right mouse button to move the camera
- Scroll to zoom
- **Add/Remove noise** sets some random cells alive/dead
- **Update interval** changes how fast generations update
- **Birth**: If a dead cell has $x ∈ \text{Birth}$ neighbors, it will be alive in the next generation. Multiple (or 0) values possible.
- **Survival**: If an alive cell has $x ∈ \text{Survival}$ neighbors, it will be alive in the next generation. Multiple (or 0) values possible.
- Standard *Game of Life* rules are set in the UI on startup (**Birth: 3 / Survival: 23**, or **B3/S23** for short)


# Fun rules to try

- **B: 3 / S: 234** 
  - forms resilient hexagons; add 5 and/or 6 to Survival for more stability
- **B: 356 / S: 0123456** and variations
  - **Fill** all cells, set **Update interval** to 2.0s and press play. You'll see intricate patterns emerging, which change at every turn and slowly devolve into chaos. 
- B: 2 / S: 3
- B: 2 / S: 34
- B: 24 / S: 
- B: 34 / S: 124
- B: 2 / S: 2345

# Run
Visit https://hylo.ink/monotile or
```sh
cargo run --release
```
