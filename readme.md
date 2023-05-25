# Run
```sh
cargo run --release
```
# UI
- "Play/Pause" button to toggle running
- "Add noise" button to bring some random cells to life (drawing cells currently not possible)
- "Birth": If a dead cell has $x ∈ \text{Birth}$ neighbors, it will be alive in the next generation. Multiple (or 0) values possible.
- "Survival": If an alive cell has $x ∈ \text{Survival}$ neighbors, it will be alive in the next generation. Multiple (or 0) values possible.
- Standard /Game of Life/ rules are set in the UI on startup
# Fun rules to try
- B: 2 / S: 3
- B: 2 / S: 34
