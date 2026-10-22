# Raylight

A small software raycasting engine written in Rust, in the style of Wolfenstein 3D.
It renders a first-person view of a 2D grid map using the CPU only.
A window and a pixel buffer come from [`minifb`](https://crates.io/crates/minifb).
Walls are textured with the original Wolfenstein 3D textures.
The PNG files are decoded with the [`png`](https://crates.io/crates/png) crate.

## How it works

1. The world is a 24x24 grid (`src/params.rs`). `0` is empty space. Other values are walls.
   A wall value `N` uses texture `N - 1`.
2. The player has a position, a direction vector, and a camera plane.
   The camera plane is perpendicular to the direction.
   Its length sets the field of view (66 degrees).
3. For each screen column, the engine casts one ray.
   The ray direction is `direction + plane * camera_x`, where `camera_x` goes from -1 to 1.
4. The DDA algorithm (`src/dda.rs`) steps the ray from one grid line to the next.
   It stops at the first wall and returns the distance and the side that was hit.
5. The wall height on screen is inversely proportional to the distance.
   The engine draws a vertical line for each column.
   The DDA step also returns the exact point where the ray hit the wall.
   This point gives the texture column. The texture row comes from the screen row.
   Walls hit on one side type are drawn darker, to give a simple lighting effect.

## Textures

The textures are the original Wolfenstein 3D wall textures.
They are stored in one sprite sheet, `wolfenstein_textures.png`.
The sheet has 10 columns and 11 rows of 64x64 pixel tiles.
The file is embedded in the executable at compile time (`include_bytes!`), so no asset files are needed at run time.
`src/textures.rs` decodes the sheet and cuts it into tiles.

The textures are the property of id Software.
They are used here for learning purposes only.

## Project layout

| File | Content |
|------|---------|
| `src/main.rs` | Window, main loop, rendering, input |
| `src/player.rs` | Player position, direction, camera plane, movement, rotation |
| `src/dda.rs` | Ray and wall intersection (DDA) |
| `src/map.rs` | Map access |
| `src/textures.rs` | Loads the texture sheet and cuts it into 64x64 tiles |
| `wolfenstein_textures.png` | Texture sheet (original Wolfenstein 3D textures) |
| `src/params.rs` | Constants: screen size, speeds, field of view, map data |

## Requirements

- Rust toolchain (edition 2024, so Rust 1.85 or newer). Install it with [rustup](https://rustup.rs).
- A desktop environment with a window system.

## Run

```sh
cargo run --release
```

## Controls

| Key | Action |
|-----|--------|
| Up | Move forward |
| Down | Move backward |
| Left / Right | Turn |
| Esc | Quit |

Movement is blocked by walls.

## Configuration

Edit `src/params.rs` to change the window size (`WIDTH`, `HEIGHT`), `VIEW_ANGLE`, `MOVE_SPEED`, `ROTATION_SPEED`, or the map.
