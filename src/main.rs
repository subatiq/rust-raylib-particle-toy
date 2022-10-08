use raylib::prelude::*;
use rand::Rng;


#[derive(Copy, Clone)]
struct Vector2 {
    x: f64,
    y: f64
}


#[derive(Copy, Clone)]
struct Particle {
    pos: Vector2,
    vel: Vector2
}


impl Particle {
    fn new(screen_width: &i32, screen_height: &i32) -> Particle {
        let mut rng = rand::thread_rng();

        Particle {
            pos: Vector2 {
                x: rng.gen_range(0.0..(screen_width - 1) as f64),
                y: rng.gen_range(0.0..(screen_height - 1) as f64)
            },
            vel: Vector2 {
                x: rng.gen_range(-100.0..100.0) / 100.0,
                y: rng.gen_range(-100.0..100.0) / 100.0,
            }
        }
    }

    fn randomize(&mut self, screen_width: &i32, screen_height: &i32) {
        let mut rng = rand::thread_rng();

        self.pos = Vector2 {
            x: rng.gen_range(0.0..(screen_width - 1) as f64),
            y: rng.gen_range(0.0..(screen_height - 1) as f64)
        };
        self.vel = Vector2 {
            x: rng.gen_range(-100.0..100.0) / 100.0,
            y: rng.gen_range(-100.0..100.0) / 100.0,
        }

    }

    fn get_dist(&self, other_pos: &Vector2) -> f64 {
        let dx = self.pos.x - other_pos.x;
        let dy = self.pos.y - other_pos.y;

        ((dx * dx) + (dy * dy)).sqrt()
    }

    fn get_normal(&self, other_pos: &Vector2) -> Vector2 {
        let dist = self.get_dist(&other_pos);

        let dx = self.pos.x - other_pos.x;
        let dy = self.pos.y - other_pos.y;

        if dist == 0.0 {
            return Vector2 {x: dx, y: dy}
        }

        Vector2 {x: dx * (1.0/dist), y: dy * (1.0/dist)}
    }

    fn attract(&mut self, pos_to_attract: &Vector2) {
        // FIXME max(dist, 0.5)
        if pos_to_attract.x < 0.0 || pos_to_attract.x > 800.0 {
            return
        }
        if pos_to_attract.y < 0.0 || pos_to_attract.y > 800.0 {
            return
        }
        let dist = self.get_dist(pos_to_attract).max(0.5);
        let normal = self.get_normal(pos_to_attract);

        self.vel.x -= normal.x / dist;
        self.vel.y -= normal.y / dist;
    }

    fn do_friction(&mut self, amount: f64) {
        self.vel.x *= amount;
        self.vel.y *= amount;
    }

    fn apply_velocity(&mut self, screen_width: &i32, screen_height: &i32) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
        }
        if self.pos.x > *screen_width as f64 {
            self.pos.x = *screen_width as f64;
        }

        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
        }
        if self.pos.y > *screen_height as f64 {
            self.pos.y = *screen_height as f64;
        }
    }

    fn draw(&self, drawer: &mut RaylibDrawHandle) {
        drawer.draw_pixel(self.pos.x as i32, self.pos.y as i32, Color::BLACK);

        //drawer.draw_circle(self.pos.x as i32, self.pos.y as i32, 0.75, Color::BLACK);
    }
}


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

        let mouse_pos = Vector2 { x: d.get_mouse_x() as f64, y: d.get_mouse_y() as f64 };

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
