mod bsp;
mod render;
mod player;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use cgmath::InnerSpace;
use std::time::{Duration, Instant};

use bsp::*;
use render::*;
use player::*;

fn main() {

    let mut keys: [bool; 256] = [false; 256];

    let mut state: i32 = 0;
    let mut render_state: i32 = 0;

    let mut ancor_point: Vertex = Vertex { x: 0.0, y: 0.0 };

    let mut mouse_pos_x: f64 = 0.0;
    let mut mouse_pos_y: f64 = 0.0;

    let mut lines: Vec<Line> = Vec::new();

    lines.push(Line::new(5.0, 50.0, 5.0, 5.0));
    lines.push(Line::new(50.0, 50.0, 5.0, 50.0));
    lines.push(Line::new(75.0, 5.0, 50.0, 50.0));
    lines.push(Line::new(5.0, 5.0, 75.0, 5.0));

    //Box
    lines.push(Line::new(15.0, 30.0, 35.0, 30.0));
    lines.push(Line::new(35.0, 18.0, 15.0, 18.0));
    lines.push(Line::new(15.0, 18.0, 15.0, 30.0));
    lines.push(Line::new(35.0, 30.0, 35.0, 18.0));

    let mut bsp = Bsp::new(&mut lines);

    let mut player = Player::new(15.0, 10.0);

    env_logger::init();
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(800, 600);
        WindowBuilder::new().with_title("BSP Test")
        .with_inner_size(size)
        .with_min_inner_size(size)
        .build(&event_loop)
        .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(VIEWWIDTH as u32, VIEWHEIGHT as u32, surface_texture).unwrap();
        pixels
    };

    let mut run_time = std::time::Instant::now();
    let mut render_time = 0.0;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                let delta_time = (((run_time.elapsed().as_nanos() as f64 / 1000.0) / 1000.0) / 1000.0) as f32;
                run_time = std::time::Instant::now();
                render_time += delta_time;
                player.vf = 0.0;
                player.vr = 0.0;
                if keys[VirtualKeyCode::W as usize] {
                    player.vf += FSPEED;
                }
                if keys[VirtualKeyCode::S as usize] {
                    player.vf -= FSPEED;
                }
                if keys[VirtualKeyCode::D as usize] {
                    player.vr -= RSPEED;
                }
                if keys[VirtualKeyCode::A as usize] {
                    player.vr += RSPEED;
                }
                player.update(delta_time, &mut bsp);
                if render_time >= 0.00666666666 {

                    clear_screen(pixels.get_frame());

                    bsp.render(pixels.get_frame(), render_state);
                    //bsp.traverse();
                    //bsp.traverse_point(Vertex { x: player.x, y: player.y });

                    let mouse_index = (mouse_pos_x as i32 + mouse_pos_y as i32 * VIEWWIDTH) as usize;

                    if mouse_index >= 0 && mouse_index < (VIEWWIDTH * VIEWHEIGHT) as usize {

                        if state > 0 {
                            let line_m = Line { vertex0: ancor_point, vertex1: Vertex { x: mouse_pos_x as f32, y: mouse_pos_y as f32 }, flags: 0};
                            draw_line(pixels.get_frame(), line_m, [50, 100, 150, 255]);
                        }
                        pixels.get_frame()[mouse_index * 4] = 255;
                    }

                    player.render(pixels.get_frame());

                    let render_result = pixels.render_with(|encoder, render_target, context| {
                        context.scaling_renderer.render(encoder, render_target);
                    });
                }
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::CursorMoved  { position, .. } => {
                        mouse_pos_x = position.x / ((window.inner_size().width / VIEWWIDTH as u32) as f64);
                        mouse_pos_y = position.y / ((window.inner_size().height / VIEWHEIGHT as u32) as f64);
                    }
                    WindowEvent::KeyboardInput {
                        input,
                        ..
                    } => {
                        let s = input.state == ElementState::Pressed;
                        let val = input.virtual_keycode.unwrap();
                        keys[val as usize] = s;
                        match input {
                            
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::E),
                                ..
                            } => {
                                if state == 0 {
                                    state = 1;
                                    ancor_point = Vertex { x: mouse_pos_x.floor() as f32, y: mouse_pos_y.floor() as f32 };
                                }
                                else if state == 1 {
                                    state = 0;
                                    let current_point = Vertex { x: mouse_pos_x.floor() as f32, y: mouse_pos_y.floor() as f32 };
                                    lines.push(Line { vertex0: ancor_point, vertex1: current_point, flags: 0 });
                                    bsp = Bsp::new(&mut lines);
                                }
                            }
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Q),
                                ..
                            } => {
                                render_state = (render_state + 1) % 3;
                            },
                            _ => {}
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    });
}