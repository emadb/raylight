use minifb::{Key, Window, WindowOptions};
use raylight::dda::Hit;
use raylight::map::Map;
use raylight::params::{HEIGHT, MAP, WIDTH};
use raylight::player::Player;
use raylight::textures::{TEX_SIZE, Textures};

fn main() {
    let map = Map::build(MAP);
    let mut player = Player::new(22, 12, 0.0);
    let textures = Textures::load_textures();
    let mut video_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        video_buffer.fill(0);

        for x in 0..WIDTH {
            let Hit {
                side,
                dist,
                u,
                cell,
            } = raylight::dda::find_wall(&player, x, &map);

            let height: i32 = ((HEIGHT as f32 / dist) as i32).max(1);
            let top = -height / 2 + (HEIGHT as i32) / 2;
            let end_y = (height / 2 + (HEIGHT as i32) / 2).min(HEIGHT as i32 - 1);
            let start_y = top.max(0);

            // Map value 0 is empty, so wall N uses texture N - 1.
            let tile = textures.get(cell as usize - 1);
            let tex_x = ((u * TEX_SIZE as f32) as usize).min(TEX_SIZE - 1);

            for y in start_y..=end_y {
                // Row inside the texture, from the unclamped wall top.
                let tex_y = ((y - top) as usize * TEX_SIZE / height as usize).min(TEX_SIZE - 1);
                let pixel = tile.pixels[tex_y * TEX_SIZE + tex_x];

                video_buffer[x + y as usize * WIDTH] = match side {
                    raylight::dda::Side::A => pixel,
                    raylight::dda::Side::B => (pixel >> 1) & 0x7F7F7F,
                };
            }
        }

        window
            .update_with_buffer(&video_buffer, WIDTH, HEIGHT)
            .unwrap();

        if window.is_key_down(Key::Up) {
            player.move_up(&map);
        }
        if window.is_key_down(Key::Down) {
            player.move_down(&map);
        }
        if window.is_key_down(Key::Left) {
            player.rotate_right();
        }
        if window.is_key_down(Key::Right) {
            player.rotate_left();
        }
    }
}
