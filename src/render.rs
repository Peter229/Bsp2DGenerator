use crate::bsp::*;

pub const VIEWWIDTH: i32 = 80;
pub const VIEWHEIGHT: i32 = 60;

//https://jstutorial.medium.com/how-to-code-your-first-algorithm-draw-a-line-ca121f9a1395
pub fn draw_line(frame: &mut [u8], line: Line, colour: [u8; 4]) {

    let x1 = line.vertex0.x as i32;
    let y1 = line.vertex0.y as i32;
    let x2 = line.vertex1.x as i32;
    let y2 = line.vertex1.y as i32;

    let dx = x2 - x1;
    let dy = y2 - y1;
    
    let dx1 = dx.abs();
    let dy1 = dy.abs();

    let mut px = 2 * dy1 - dx1;
    let mut py = 2 * dx1 - dy1;

    let mut x = 0;
    let mut y = 0;
    let mut xe = 0;
    let mut ye = 0;

    if dy1 <= dx1 {

        if dx >= 0 {
            x = x1;
            y = y1;
            xe = x2;
        }
        else {
            x = x2;
            y = y2;
            xe = x1;
        }
        
        frame[((x + y * VIEWWIDTH) * 4) as usize] = colour[0];
        frame[(((x + y * VIEWWIDTH) * 4) + 1) as usize] = colour[1];
        frame[(((x + y * VIEWWIDTH) * 4) + 2) as usize] = colour[2];
        frame[(((x + y * VIEWWIDTH) * 4) + 3) as usize] = colour[3];

        while x < xe {
            x = x + 1;

            if px < 0 {
                px = px + 2 * dy1;
            }
            else {
                if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                    y = y + 1;
                }
                else {
                    y = y - 1;
                }
                px = px + 2 * (dy1 - dx1);
            }

            frame[((x + y * VIEWWIDTH) * 4) as usize] = colour[0];
            frame[(((x + y * VIEWWIDTH) * 4) + 1) as usize] = colour[1];
            frame[(((x + y * VIEWWIDTH) * 4) + 2) as usize] = colour[2];
            frame[(((x + y * VIEWWIDTH) * 4) + 3) as usize] = colour[3];
        }
    }
    else {

        if dy >= 0 {
            x = x1;
            y = y1;
            ye = y2;
        }
        else {
            x = x2;
            y = y2;
            ye = y1;
        }

        frame[((x + y * VIEWWIDTH) * 4) as usize] = colour[0];
        frame[(((x + y * VIEWWIDTH) * 4) + 1) as usize] = colour[1];
        frame[(((x + y * VIEWWIDTH) * 4) + 2) as usize] = colour[2];
        frame[(((x + y * VIEWWIDTH) * 4) + 3) as usize] = colour[3];

        while y < ye {
            y = y + 1;

            if py <= 0 {
                py = py + 2 * dx1;
            }
            else {
                if (dx < 0 && dy < 0) || (dx > 0 && dy > 0) {
                    x = x + 1;
                }
                else {
                    x = x - 1;
                }
                py = py + 2 * (dx1 - dy1);
            }

            frame[((x + y * VIEWWIDTH) * 4) as usize] = colour[0];
            frame[(((x + y * VIEWWIDTH) * 4) + 1) as usize] = colour[1];
            frame[(((x + y * VIEWWIDTH) * 4) + 2) as usize] = colour[2];
            frame[(((x + y * VIEWWIDTH) * 4) + 3) as usize] = colour[3];
        }
    }
}

pub fn clear_screen(frame: &mut [u8]) {
    
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

        let rgba = [0, 0, 0, 255];

        pixel.copy_from_slice(&rgba);
    }
}