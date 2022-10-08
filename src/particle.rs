use rand::Rng;
use raylib::prelude::*;


#[derive(Copy, Clone)]
pub struct Particle {
    pos: Vector2,
    vel: Vector2
}


impl Particle {
    pub fn new(screen_width: &i32, screen_height: &i32) -> Particle {
        let mut rng = rand::thread_rng();

        Particle {
            pos: Vector2 {
                x: rng.gen_range(0.0..(screen_width - 1) as f32),
                y: rng.gen_range(0.0..(screen_height - 1) as f32)
            },
            vel: Vector2 {
                x: rng.gen_range(-100.0..100.0) / 100.0,
                y: rng.gen_range(-100.0..100.0) / 100.0,
            }
        }
    }

    pub fn randomize(&mut self, screen_width: &i32, screen_height: &i32) {
        let mut rng = rand::thread_rng();

        self.pos = Vector2 {
            x: rng.gen_range(0.0..(screen_width - 1) as f32),
            y: rng.gen_range(0.0..(screen_height - 1) as f32)
        };
        self.vel = Vector2 {
            x: rng.gen_range(-100.0..100.0) / 100.0,
            y: rng.gen_range(-100.0..100.0) / 100.0,
        }

    }

    fn get_dist(&self, other_pos: &Vector2) -> f32 {
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

    pub fn attract(&mut self, pos_to_attract: &Vector2) {
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

    pub fn do_friction(&mut self, amount: f32) {
        self.vel.x *= amount;
        self.vel.y *= amount;
    }

    pub fn apply_velocity(&mut self, screen_width: &i32, screen_height: &i32) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;

        if self.pos.x < 0.0 {
            self.pos.x = 0.0;
        }
        if self.pos.x > *screen_width as f32 {
            self.pos.x = *screen_width as f32;
        }

        if self.pos.y < 0.0 {
            self.pos.y = 0.0;
        }
        if self.pos.y > *screen_height as f32 {
            self.pos.y = *screen_height as f32;
        }
    }

    pub fn draw(&self, drawer: &mut RaylibDrawHandle) {
        drawer.draw_pixel(self.pos.x as i32, self.pos.y as i32, Color::BLACK);
    }
}
