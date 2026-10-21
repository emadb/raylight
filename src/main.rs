use minifb::{Key, Window, WindowOptions};
use raylight::map::Map;
use raylight::params::{HEIGHT, MAP, VIEW_ANGLE, WIDTH};
use raylight::player::Player;


fn main() {

    let map = Map::build(MAP);
    let mut player = Player::new(22, 12, 0.0);
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

    let k = (VIEW_ANGLE / 2.0).to_radians().tan();

    let dir_x = player.direction.sin();
    let dir_y = -player.direction.cos();

    let plane_x: f32 = k * player.direction.cos() ;
    let plane_y: f32 = k * player.direction.sin();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        video_buffer.fill(0);

        for x in 0..WIDTH {

            let camera_x = 2.0 * (x as f32 + 0.5) / WIDTH as f32 - 1.0;

            let ray_direction_x = dir_x + plane_x * camera_x;
            let ray_direction_y = dir_y + plane_y * camera_x;

            let (side, dist) = find_wall((ray_direction_x, ray_direction_y), (player.x, player.y), &map);

            let height: i32 = (HEIGHT as f32 / dist.max(0.0001)) as i32;
            let start_y = -height / 2 + (HEIGHT as i32) / 2;
            let end_y = height / 2 + (HEIGHT as i32) / 2;

            let start_y = if start_y >= 0 { start_y } else { 0 };
            let end_y = if end_y >= HEIGHT as i32 { HEIGHT as i32 - 1 } else { end_y };

            let color = if side == 1 {29000} else {29000 / 2};
            for y in start_y..=end_y {
                video_buffer[x + y as usize * WIDTH] = color;
            }
        }

        window
            .update_with_buffer(&video_buffer, WIDTH, HEIGHT)
            .unwrap();

        let move_speed = 0.08; //60.0 * 5.0; //the constant value is in squares/second
        let rot_s: f32 = 0.04; //the constant value is in radians/second

        if window.is_key_down(Key::Up) {
            let xx = player.x + dir_x * move_speed;
            let yy = player.y + dir_y * move_speed;
            if map.get(xx.floor() as i32, player.y.floor() as i32) == 0 { player.x += dir_x * move_speed}
            if map.get(player.x.floor() as i32, yy.floor() as i32) == 0 { player.y += dir_y * move_speed}
        }
     }
}

fn find_wall((dir_x, dir_y): (f32, f32), (px, py): (f32, f32), map: &Map) -> (u32, f32) {
    let  mut wall_found = false;
    let mut current_x = px.floor() as i32;
    let mut current_y = py.floor() as i32;

    let delta_dist_x = if dir_x == 0.0 {f32::INFINITY } else  { (1.0 / dir_x).abs() };
    let delta_dist_y = if dir_y == 0.0 {f32::INFINITY } else  { (1.0 / dir_y).abs() };

    let  (step_x, mut side_dist_x) = if dir_x < 0.0 {
        (-1, (px - current_x as f32) * delta_dist_x)
    } else {
        (1, (px + 1.0 - current_x as f32) * delta_dist_x)
    };

    let (step_y, mut side_dist_y) = if dir_y < 0.0 {
        (-1, (py - current_y as f32) * delta_dist_y)
    } else {
        (1, (py + 1.0 - current_y as f32) * delta_dist_y)
    };
    let mut side = 0;

    while !wall_found {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            current_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            current_y += step_y;
            side = 1;
        }

        if map.get(current_x, current_y) != 0 { wall_found = true; }
    }

    let dist = if side == 0 {
        side_dist_x - delta_dist_x
    } else {
        side_dist_y - delta_dist_y
    };

    (side, dist)
}
