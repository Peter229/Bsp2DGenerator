use crate::render::*;

pub struct ColPacket {
    pub happen: bool,
    pub x: f32,
    pub y: f32,
    pub t: f32,
    pub d_x: f32,
    pub d_y: f32,
    pub n_x: f32,
    pub n_y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Copy, Clone)]
pub struct Line {
    pub vertex0: Vertex,
    pub vertex1: Vertex,
    pub flags: i32,
}

impl Line {
    
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Line {

        Line { vertex0: Vertex { x: x0, y: y0 }, vertex1: Vertex { x: x1, y: y1 }, flags: 0}
    }

    pub fn line_intersection(&self, line: &Line) -> (bool, f32, f32, f32) {

        let r_x = self.vertex1.x - self.vertex0.x;
        let r_y = self.vertex1.y - self.vertex0.y;

        let s_x = line.vertex1.x - line.vertex0.x;
        let s_y = line.vertex1.y - line.vertex0.y;

        let rxs = r_x * s_y - r_y * s_x;

        let qp_x = line.vertex0.x - self.vertex0.x;
        let qp_y = line.vertex0.y - self.vertex0.y;
        let qpxr = qp_x * r_y - qp_y * r_x;

        if rxs == 0.0 && qpxr == 0.0 {

            return (false, 0.0, 0.0, 0.0);
        }

        if rxs == 0.0 && qpxr != 0.0 {
            return (false, 0.0, 0.0, 0.0);
        }

        let t = (qp_x * s_y - qp_y * s_x) / rxs;

        let u = qpxr / rxs;

        //uncomment for segment test
        if rxs != 0.0 && /*(0.0 <= t && t <= 1.0) &&*/ (0.0 <= u && u <= 1.0) {

            let out_x = self.vertex0.x + t * r_x;
            let out_y = self.vertex0.y + t * r_y;
            return (true, out_x, out_y, t);
        }

        return (false, 0.0, 0.0, 0.0);
    }

    pub fn segment_intersection(&self, line: &Line) -> (bool, f32, f32, f32) {

        let r_x = self.vertex1.x - self.vertex0.x;
        let r_y = self.vertex1.y - self.vertex0.y;

        let s_x = line.vertex1.x - line.vertex0.x;
        let s_y = line.vertex1.y - line.vertex0.y;

        let rxs = r_x * s_y - r_y * s_x;

        let qp_x = line.vertex0.x - self.vertex0.x;
        let qp_y = line.vertex0.y - self.vertex0.y;
        let qpxr = qp_x * r_y - qp_y * r_x;

        if rxs == 0.0 && qpxr == 0.0 {

            return (false, 0.0, 0.0, 0.0);
        }

        if rxs == 0.0 && qpxr != 0.0 {
            return (false, 0.0, 0.0, 0.0);
        }

        let t = (qp_x * s_y - qp_y * s_x) / rxs;

        let u = qpxr / rxs;

        if rxs != 0.0 && (0.0 <= t && t <= 1.0) && (0.0 <= u && u <= 1.0) {

            let out_x = self.vertex0.x + t * r_x;
            let out_y = self.vertex0.y + t * r_y;
            return (true, out_x, out_y, t);
        }

        return (false, 0.0, 0.0, 0.0);
    }

    pub fn check_side(&self, line: &Line) -> (Option<Line>, Option<Line>, Option<Line>) {

        let is_front_0 = (self.vertex1.x - self.vertex0.x) * (line.vertex0.y - self.vertex0.y) - (self.vertex1.y - self.vertex0.y) * (line.vertex0.x - self.vertex0.x);
        let is_front_1 = (self.vertex1.x - self.vertex0.x) * (line.vertex1.y - self.vertex0.y) - (self.vertex1.y - self.vertex0.y) * (line.vertex1.x - self.vertex0.x);
        
        let mut front_lines = None;
        let mut back_lines = None;

        if is_front_0 == 0.0 && is_front_1 == 0.0 {
            //Add to coplaner list
            return (Some(*line), None, None);
        }
        else if is_front_0 > 0.0 && is_front_1 > 0.0 {
            return (None, Some(*line), None);
        }
        else if is_front_0 < 0.0 && is_front_1 < 0.0 {
            return (None, None, Some(*line));
        }
        else if (is_front_0 > 0.0 && is_front_1 < 0.0) || (is_front_0 < 0.0 && is_front_1 > 0.0) {
            let a1 = self.vertex1.y - self.vertex0.y;
            let b1 = self.vertex0.x - self.vertex1.x;
            let c1 = a1 * self.vertex0.x + b1 * self.vertex0.y;

            let a2 = line.vertex1.y - line.vertex0.y;
            let b2 = line.vertex0.x - line.vertex1.x;
            let c2 = a2 * line.vertex0.x + b2 * line.vertex0.y;

            let det = a1 * b2 - a2 * b1;

            if det == 0.0 {
                println!("Error should not reach, lines are parallel");
            }
            else {
                let x = (b2 * c1 - b1 * c2) / det;
                let y = (a1 * c2 - a2 * c1) / det;
                let mut line_front = if is_front_0 > 0.0 && is_front_1 < 0.0 {
                    Line { vertex0: line.vertex0, vertex1: Vertex { x, y }, flags: line.flags }
                }
                else {
                    Line { vertex0: Vertex { x, y }, vertex1: line.vertex1, flags: line.flags }
                };
                let mut line_back = if is_front_0 > 0.0 && is_front_1 < 0.0 {
                    Line { vertex0: Vertex { x, y }, vertex1: line.vertex1, flags: line.flags }
                }
                else {
                    Line { vertex0: line.vertex0, vertex1: Vertex { x, y }, flags: line.flags }
                };

                
                let mut dx1 = line_front.vertex1.x - line_front.vertex0.x;
                let mut dy1 = line_front.vertex1.y - line_front.vertex0.y;
                let mag1 = (dx1*dx1 + dy1*dy1).sqrt();

                let mut dx2 = line_back.vertex1.x - line_back.vertex0.x;
                let mut dy2 = line_back.vertex1.y - line_back.vertex0.y;
                let mag2 = (dx2*dx2 + dy2*dy2).sqrt();

                if (mag1 == 0.0 || mag1.is_nan()) && (mag2 == 0.0 || mag2.is_nan()) {
                    return (None, None, None);
                }
                else if !(mag1 == 0.0 || mag1.is_nan()) && !(mag2 == 0.0 || mag2.is_nan()) {
                    return (None, Some(line_front), Some(line_back));
                }
                else if !(mag1 == 0.0 || mag1.is_nan()) {
                    return (None, Some(line_front), None);
                }
                else {
                    return (None, None, Some(line_back));
                }
            }
        }
        else if is_front_0 > 0.0 && is_front_1 == 0.0 {
            return (None, Some(*line), None);
        }
        else if is_front_0 < 0.0 && is_front_1 == 0.0 {
            return (None, None, Some(*line));
        }
        else if is_front_0 == 0.0 && is_front_1 > 0.0 {
            return (None, Some(*line), None);
        }
        else if is_front_0 == 0.0 && is_front_1 < 0.0 {
            return (None, None, Some(*line));
        }

        println!("Should not make it here");
        return (None, front_lines, back_lines);
    }
}

pub struct Node {
    co_planar: Vec<Line>,
    front: Vec<Node>,
    back: Vec<Node>,
    colour: [u8; 4],
    r_w_h: i32,
}

impl Node {

    pub fn new(line_list: &mut Vec<Line>) -> Node {

        let mut best_splitter = 0;

        if line_list.len() > 1 {
            let mut diff = 0i32;
            let mut min_diff = i32::MAX;
            for j in 0..line_list.len() {
                diff = 0;
                for i in 0..line_list.len() {
                    if i == j {
                        continue;
                    }
                    let (line, front_line, back_line) = line_list[j].check_side(&line_list[i]);
                    if front_line.is_some() {
                        diff += 1;
                    }
                    if back_line.is_some() {
                        diff -= 1;
                    }
                }
                if diff.abs() < min_diff {
                    min_diff = diff.abs();
                    best_splitter = j;
                }
            }
        }

        let mut co_planar = Vec::new();
        co_planar.push(line_list[best_splitter]);

        //Create convex
        /*let d_x = co_planar[0].vertex1.x - co_planar[0].vertex0.x;
        let d_y = co_planar[0].vertex1.y - co_planar[0].vertex0.y;

        let far_x = co_planar[0].vertex1.x + (d_x * 100.0);
        let far_y = co_planar[0].vertex1.y + (d_y * 100.0);

        let close_x = co_planar[0].vertex0.x - (d_x * 100.0);
        let close_y = co_planar[0].vertex0.y - (d_y * 100.0);

        let close_line = Line { vertex0: co_planar[0].vertex0, vertex1: Vertex { x: close_x, y: close_y }, flags: 1 };
        let far_line = Line { vertex0: co_planar[0].vertex1, vertex1: Vertex { x: far_x, y: far_y }, flags: 1 };

        for i in 0..line_list.len() {
            if best_splitter == i {
                continue;
            }

            let (happen, x, y, t) = close_line.line_intersection(&line_list[i]);
            if happen && t.abs() > 0.0 {
                line_list.push(Line { vertex0: Vertex { x, y }, vertex1: co_planar[0].vertex0, flags: 1 });
            }

            let (happen2, x2, y2, t2) = far_line.line_intersection(&line_list[i]);
            if happen2 && t2.abs() > 0.0 {
                //rintln!("{:?} {} {}", co_planar[0].vertex1, x2, y2);
                line_list.push(Line { vertex0: co_planar[0].vertex1, vertex1: Vertex { x: x2, y: y2 }, flags: 1 });
            }
        }*/
        

        let mut front_lines = Vec::new();
        let mut back_lines = Vec::new();

        let mut front: Vec<Node> = Vec::new();
        let mut back: Vec<Node> = Vec::new();

        if line_list.len() > 1 {
            for i in 0..line_list.len() {
                if best_splitter == i {
                    continue;
                }
                let (line, front_line, back_line) = co_planar[0].check_side(&line_list[i]);
                if line.is_some() {
                    front_lines.push(line.unwrap());
                }
                if front_line.is_some() {
                    front_lines.push(front_line.unwrap());
                }
                if back_line.is_some() {
                    back_lines.push(back_line.unwrap());
                }
            }
        }

        if front_lines.len() != 0 {
            front.push(Node::new(&mut front_lines));
        }

        if back_lines.len() != 0 {
            back.push(Node::new(&mut back_lines));
        }

        let colour = [rand::random::<u8>(), rand::random::<u8>(), rand::random::<u8>(), 255];

        Node { co_planar, front, back, colour, r_w_h: 0 }
    }

    pub fn traverse(&self) {

        for i in 0..self.front.len() {
            self.front[i].traverse();
        }
        for i in 0..self.back.len() {
            self.back[i].traverse();
        }
    }

    pub fn traverse_point(&mut self, point: Vertex) {

        let point_val = (self.co_planar[0].vertex1.x - self.co_planar[0].vertex0.x) * (point.y - self.co_planar[0].vertex0.y) - (self.co_planar[0].vertex1.y - self.co_planar[0].vertex0.y) * (point.x - self.co_planar[0].vertex0.x);
        if point_val > 0.0 {
            if self.front.len() == 0 {
                //leaf
            }
            else {
                self.front[0].traverse_point(point);
            }
        }
        else if point_val < 0.0 {
            if self.back.len() == 0 {
                //leaf
            }
            else {
                self.back[0].traverse_point(point);
            }
        }
        else {

        }
    }

    pub fn check_line(&mut self, line: &Line, col_packet: &mut ColPacket) -> bool {

        let (line_c, front_line, back_line) = self.co_planar[0].check_side(line);
        if (front_line.is_some() && back_line.is_some()) || line_c.is_some() {

            if self.front.len() > 0 {
                if self.front[0].check_line(line, col_packet) {
                    return true;
                }
            }
            if self.back.len() > 0 {
                if self.back[0].check_line(line, col_packet) {
                    return true;
                }
            }

            if self.co_planar[0].flags == 1 {
                return false;
            }

            let mut dx = self.co_planar[0].vertex1.x - self.co_planar[0].vertex0.x;
            let mut dy = self.co_planar[0].vertex1.y - self.co_planar[0].vertex0.y;
            let mag = (dx*dx + dy*dy).sqrt();
            let n_x = (-dy / mag);
            let n_y = (dx / mag);

            let (b, x, y, t) = line.line_intersection(&self.co_planar[0]);
            if b {
                col_packet.x = x;
                col_packet.y = y;

                col_packet.happen = true;
                col_packet.t = t;
                col_packet.n_x = n_x;
                col_packet.n_y = n_y;
                return true;
            }
            return false;
        }
        else if front_line.is_some() {
            if self.front.len() == 0 {
                return false;
            }
            else {
                return self.front[0].check_line(line, col_packet);
            }
        }
        else if back_line.is_some() {
            if self.back.len() == 0 {

                return false;
                //println!("No collision");
            }
            else {
                return self.back[0].check_line(line, col_packet);
            }
        }

        false
    }

    pub fn render(&mut self, frame: &mut [u8], render_state: i32) {

        for i in 0..self.co_planar.len() {

            let mid_point_x = (self.co_planar[i].vertex0.x + self.co_planar[i].vertex1.x) / 2.0;
            let mid_point_y = (self.co_planar[i].vertex0.y + self.co_planar[i].vertex1.y) / 2.0;

            let mut dx = self.co_planar[i].vertex1.x - self.co_planar[i].vertex0.x;
            let mut dy = self.co_planar[i].vertex1.y - self.co_planar[i].vertex0.y;

            let mag = (dx*dx + dy*dy).sqrt();

            let mut dir_x = (-dy / mag) * 2.0;
            let mut dir_y = (dx / mag) * 2.0;

            let n = Line { vertex0: Vertex { x: mid_point_x, y: mid_point_y }, vertex1: Vertex { x: mid_point_x + dir_x, y: mid_point_y + dir_y }, flags: 0 };


            if self.r_w_h == 1 {
                draw_line(frame, self.co_planar[i], [100, 255, 100, 255]);
                self.r_w_h = 0;
            }
            else if self.co_planar[i].flags == 0 {
                draw_line(frame, self.co_planar[i], self.colour);
                if render_state == 1 {
                    draw_line(frame, n, [0, 0, 255, 0]);
                }
            }
            else if self.co_planar[i].flags == 1 && render_state == 2 {
                draw_line(frame, self.co_planar[i], [100, 100, 100, 10]);
            }
        }

        for i in 0..self.front.len() {
            self.front[i].render(frame, render_state);
        }
        for i in 0..self.back.len() {
            self.back[i].render(frame, render_state);
        }
    }
}

pub struct Bsp {
    nodes: Node,
}

impl Bsp {

    pub fn new(line_list: &mut Vec<Line>) -> Bsp {

        let mut new_line_list = Vec::new();
        for i in 0..line_list.len() {
            new_line_list.push(line_list[i]);
        }

        for i in 0..new_line_list.len() {

            let mut min_t1 = f32::MAX;
            let mut min_t2 = f32::MAX;

            let mut line_front = Line { vertex0: Vertex { x: 0.0, y: 0.0 }, vertex1: Vertex { x: 0.0, y: 0.0 }, flags: 1 };
            let mut line_back = Line { vertex0: Vertex { x: 0.0, y: 0.0 }, vertex1: Vertex { x: 0.0, y: 0.0 }, flags: 1 };

            for j in 0..new_line_list.len() {
                if j == i {
                    continue;
                }

                let d_x = new_line_list[i].vertex1.x - new_line_list[i].vertex0.x;
                let d_y = new_line_list[i].vertex1.y - new_line_list[i].vertex0.y;
        
                let far_x = new_line_list[i].vertex1.x + d_x;
                let far_y = new_line_list[i].vertex1.y + d_y;
        
                let close_x = new_line_list[i].vertex0.x - d_x;
                let close_y = new_line_list[i].vertex0.y - d_y;
        
                let close_line = Line { vertex0: new_line_list[i].vertex0, vertex1: Vertex { x: close_x, y: close_y }, flags: 1 };
                let far_line = Line { vertex0: new_line_list[i].vertex1, vertex1: Vertex { x: far_x, y: far_y }, flags: 1 };

                let (happen, x, y, t) = close_line.line_intersection(&new_line_list[j]);
                if happen && t > 0.0 {
                    if t < min_t1 {
                        line_front = Line { vertex0: Vertex { x, y }, vertex1: new_line_list[i].vertex0, flags: 1 };
                        min_t1 = t;
                    }
                    //new_line_list.push(Line { vertex0: Vertex { x, y }, vertex1: new_line_list[i].vertex0, flags: 1 });
                }

                let (happen2, x2, y2, t2) = far_line.line_intersection(&new_line_list[j]);
                if happen2 && t2 > 0.0 {
                    if t2 < min_t2 {
                        line_back = Line { vertex0: new_line_list[i].vertex1, vertex1: Vertex { x: x2, y: y2 }, flags: 1 };
                        min_t2 = t2;
                    }
                }
            }

            if min_t1 != f32::MAX {
                new_line_list.push(line_front);
            }
            if min_t2 != f32::MAX {
                new_line_list.push(line_back);
            }
        }

        let nodes: Node = Node::new(&mut new_line_list);

        Bsp { nodes }
    }

    pub fn traverse(&self) {
        self.nodes.traverse();
    }

    pub fn traverse_point(&mut self, point: Vertex) {
        self.nodes.traverse_point(point);
    }

    pub fn check_line_intersection(&mut self, line: &Line) -> ColPacket {

        let mut col_packet = ColPacket { happen: false, x: 0.0, y: 0.0, t: 0.0, d_x: 0.0, d_y: 0.0, n_x: 0.0, n_y: 0.0 };

        self.nodes.check_line(line, &mut col_packet);

        col_packet
    }

    pub fn render(&mut self, frame: &mut [u8], render_state: i32) {
        self.nodes.render(frame, render_state);
    }
}