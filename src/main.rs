use minifb::{Key, Window, WindowOptions};
use raylight::map::Map;
use raylight::params::{HEIGHT, MAP, VIEW_ANGLE, WIDTH};
use raylight::player::Player;


fn main() {

    let map = Map::build(MAP);
    let player = Player::new(22, 12, 0.0);
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

    let k = (VIEW_ANGLE.to_radians()).tan();

    let dir_x = player.direction.sin();
    let dir_y = -player.direction.cos();

    let plane_x: f32 = k * player.direction.cos() ;
    let plane_y: f32 = k * player.direction.sin();



    while window.is_open() && !window.is_key_down(Key::Escape) {
        video_buffer.fill(0);

        for x in 0..WIDTH {

            let camera_x = 2.0 * (x as f32 + 0.5) / WIDTH as f32 - 1.0; // Converte una coordinata orizzontale dello schermo (da 0 a 640) in un valore compreso fra -1 e 1.

            let ray_direction_x = dir_x + plane_x * camera_x;
            let ray_direction_y = dir_y + plane_y * camera_x;

            let distance = find_wall((ray_direction_x, ray_direction_y), (player.x, player.y), &map);
            println!("D = {distance:?}");
        }

        window
            .update_with_buffer(&video_buffer, WIDTH, HEIGHT)
            .unwrap();
     }
}

fn find_wall((dir_x, dir_y): (f32, f32), (px, py): (u32, u32), map: &Map) -> (f32, f32) {
    let  mut wall_found = false;
    let mut current_x = px as f32;
    let mut current_y = py as f32;

    let slope = dir_y / dir_x;

    while !wall_found && current_x < WIDTH as f32 && current_y < HEIGHT as f32 {
        if slope <= 1.0 {
            current_x += 1.0;
            current_y += slope;
        } else {
            current_x += 1.0 / slope;
            current_y += 1.0;
        }
        let r = current_x.floor() as usize;
        let c = current_y.floor() as usize;
        if map.get(r, c) != 0 { wall_found = true; }
    }

    (current_x, current_y)
}
