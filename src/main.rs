use minifb::{Key, Window, WindowOptions};
use raylight::map::Map;
use raylight::params::{HEIGHT, MAP, WIDTH};
use raylight::player::Player;


fn main() {

    let map = Map::build(MAP);
    let player = Player::new(22, 12, 0);
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

        window
            .update_with_buffer(&video_buffer, WIDTH, HEIGHT)
            .unwrap();
     }
}
