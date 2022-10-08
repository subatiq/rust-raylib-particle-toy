mod particle;
use raylib::prelude::*;

use particle::Particle;

fn main() {
    let screen_width = 800;
    let screen_height = 800;

    const PARTICLE_COUNT: usize = 100000;
    let mut particles = [Particle::new(&screen_width, &screen_height); PARTICLE_COUNT];

    for i in 0..PARTICLE_COUNT {
        particles[i].randomize(&screen_width, &screen_height)
    }

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("Raylib particle toy")
        .build();


    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        let mouse_pos = Vector2 { x: d.get_mouse_x() as f32, y: d.get_mouse_y() as f32};

        d.clear_background(Color::WHITE);

        for i in 0..PARTICLE_COUNT {
            particles[i].attract(&mouse_pos);
            particles[i].do_friction(0.99);
            particles[i].apply_velocity(&screen_width, &screen_height);

            particles[i].draw(&mut d);
        }

        d.draw_fps(10, 10);
    }
}
