use crate::render::*;
use crate::bsp::*;

pub const FSPEED: f32 = 70.0;
pub const RSPEED: f32 = 6.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub vf: f32,
    pub vr: f32,
}

impl Player {

    pub fn new(x: f32, y: f32) -> Player {
        Player { x, y, angle: 0.0, vf: 0.0, vr: 0.0 }
    }

    pub fn render(&self, frame: &mut [u8]) {

        let x_d = (self.angle.sin() * 3.0) + self.x;
        let y_d = (self.angle.cos() * 3.0) + self.y;

        let l = Line { vertex0: Vertex { x: self.x, y: self.y }, vertex1: Vertex { x: x_d, y: y_d}, flags: 0 };

        draw_line(frame, l, [0, 255, 0, 255]);

        frame[((self.x as i32 + self.y as i32 * VIEWWIDTH) * 4) as usize] = 255;
    }

    pub fn update(&mut self, delta: f32, bsp: &mut Bsp) {

        self.angle += self.vr * delta;

        let mut wish_x = self.x + self.angle.sin() * self.vf * delta;
        let mut wish_y = self.y + self.angle.cos() * self.vf * delta;

        let mut in_x = self.x;
        let mut in_y = self.y;

        let mut push_back_modi = 1.0;

        let mut prev_n_x = 0.0;
        let mut prev_n_y = 0.0;

        for i in 0..6 {

            let l_t = Line { vertex0: Vertex { x: in_x, y: in_y }, vertex1: Vertex { x: wish_x, y: wish_y }, flags: 0 };
            let mut dx = wish_x - in_x;
            let mut dy = wish_y - in_y;
            let mag = (dx*dx + dy*dy).sqrt();
            dx = dx / mag;
            dy = dy / mag;

            let col_info = bsp.check_line_intersection(&l_t);

            if col_info.happen {
                let mut offset_x = col_info.x + (col_info.n_x / (2.0 / push_back_modi));
                let mut offset_y = col_info.y + (col_info.n_y / (2.0 / push_back_modi));
                //Fix for acute edges
                if i > 0 {
                    let dot = col_info.n_x * prev_n_x + col_info.n_y * prev_n_y;
                    if dot < 0.0 {
                        let n_x = (col_info.n_x + prev_n_x) / 2.0;
                        let n_y = (col_info.n_y + prev_n_y) / 2.0;
                        let mag = (n_x * n_x + n_y * n_y).sqrt();
                        let n_x_n = (n_x / mag);
                        let n_y_n = (n_y / mag);
                        offset_x = col_info.x + (n_x_n / 2.0);
                        offset_y = col_info.y + (n_y_n / 2.0);
                    }

                    if i >= 5 {
                        println!("Ou ho");
                    }
                }

                wish_x = offset_x;
                wish_y = offset_y;
                prev_n_x = col_info.n_x;
                prev_n_y = col_info.n_y;
                //in_x = in_x + dx * (col_info.t - 0.001);
                //in_y = in_y + dy * (col_info.t - 0.001);
            }
            else {
                break;
            }
        }

        self.x = wish_x;
        self.y = wish_y;
    }
}