use std::{
    f32::consts::{FRAC_PI_8, PI, SQRT_2},
    num,
};

use macroquad::prelude::*;

const CANVAS_WIDTH: u16 = 480;
const CANVAS_HEIGHT: u16 = 720;
const BORDER_THICKNESS: u16 = 5;
const BORDER_COLOR: Color = GRAY;

const TURN_SPEED: f32 = PI / 128.;
const DEFAULT_LINE_SPEED: f32 = 30.;
const PLAYER_RADIUS: usize = 5;

struct Rgba<T: std::cmp::PartialEq> {
    r: T,
    g: T,
    b: T,
    a: T,
}
impl<T: std::cmp::PartialEq> Rgba<T> {
    pub fn new(r: T, g: T, b: T, a: T) -> Self {
        Rgba { r, g, b, a }
    }
    pub fn alphaless_match(&self, other: &Rgba<T>) -> bool {
        return self.r == other.r && self.g == other.g && self.b == other.b;
    }
}

struct GameCanvas {
    canvas: Vec<u8>,
}

struct Direction {
    x: f32,
    y: f32,
}

impl Direction {
    pub fn from_angle(angle: f32) -> Self {
        let sc = angle.sin_cos();
        Self::new(sc.0, sc.1)
    }
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

struct Point<T> {
    x: T,
    y: T,
}

struct Player {
    location: Point<usize>,
    direction: Direction,
    color: Color,
}

#[macroquad::main("Curve Game")]
async fn main() {
    println!("Hello, world!");
    // let canvas = [
    //     255, 0, 0, 192, 0, 255, 0, 192, 0, 0, 255, 192, 0, 255, 255, 192,
    // ];
    // fn make_canvas(i: usize) -> u8 {
    //     match i % 4 {
    //         0 => 0,
    //         1 => 0,
    //         2 => 0,
    //         3 => 255,
    //         _ => unreachable!("asdadfkhd Unreachable adscvx"),
    //     }
    // }
    // let canvas: [u8; 4 * CANVAS_HEIGHT as usize * CANVAS_WIDTH as usize] =
    //     core::array::from_fn(make_canvas);
    // let mut canvas: Vec<u8> = Vec::with_capacity(4*CANVAS_HEIGHT as usize * CANVAS_WIDTH as usize);
    // let canvas: Vec<u8> = vec![0; 4 * CANVAS_HEIGHT as usize * CANVAS_WIDTH as usize];
    // let mut canvas: Vec<u8> = canvas.iter().map(make_canvas).collect();

    let mut canvas = GameCanvas {
        canvas: (0..CANVAS_HEIGHT as usize * CANVAS_WIDTH as usize)
            .flat_map(|_| [0, 0, 0, 255])
            .collect(),
    };

    // let render_target = render_target(CANVAS_WIDTH as u32, CANVAS_HEIGHT as u32);
    // render_target.texture.set_filter(FilterMode::Nearest);

    // println!("first addr is {:p}", &render_target.texture);
    let mut direction_angle = 0.0f32;
    let mut location = Vec2::new(100., 100.);
    let mut line_speed_multiplier = 1.;
    loop {
        clear_background(WHITE);
        draw_ui();

        if is_key_down(KeyCode::A) {
            direction_angle += TURN_SPEED % (2. * PI);
        }
        if is_key_down(KeyCode::D) {
            direction_angle -= TURN_SPEED % (2. * PI);
        }
        if is_key_down(KeyCode::Escape) {
            return;
        }

        let direction = Direction::from_angle(direction_angle);
        location.x += direction.x * line_speed_multiplier * DEFAULT_LINE_SPEED * get_frame_time();
        location.y += direction.y * line_speed_multiplier * DEFAULT_LINE_SPEED * get_frame_time();
        // canvas.draw_rectangle(
        //     location.x as usize - PLAYER_RADIUS,
        //     location.y as usize - PLAYER_RADIUS,
        //     2 * PLAYER_RADIUS,
        //     2 * PLAYER_RADIUS,
        //     Rgba::new(255, 0, 200, 255),
        // );
        canvas.draw_circle(
            location.x as usize,
            location.y as usize,
            PLAYER_RADIUS,
            Rgba::new(255, 0, 200, 255),
        );
        draw_texture(
            &Texture2D::from_rgba8(CANVAS_WIDTH, CANVAS_HEIGHT, &canvas.canvas),
            BORDER_THICKNESS as f32,
            BORDER_THICKNESS as f32,
            LIGHTGRAY,
        );
        draw_circle(
            location.x + BORDER_THICKNESS as f32,
            location.y + BORDER_THICKNESS as f32,
            PLAYER_RADIUS as f32,
            RED,
        );

        let next_draw_pixel_location = (
            (location.x + direction.x * PLAYER_RADIUS as f32 * (1.2)) as usize,
            (location.y + direction.y * PLAYER_RADIUS as f32 * (1.2)) as usize,
        );
        let next_draw_center_pixel =
            canvas.at(next_draw_pixel_location.0, next_draw_pixel_location.1);
        if !next_draw_center_pixel.alphaless_match(&Rgba::new(0, 0, 0, 0)) {
            line_speed_multiplier = 0.;
            draw_text("Owie :(", 200., 200., 40., WHITE);
        }

        draw_circle(
            next_draw_pixel_location.0 as f32 + BORDER_THICKNESS as f32,
            next_draw_pixel_location.1 as f32 + BORDER_THICKNESS as f32,
            2.,
            LIME,
        );

        // if temp % 10 == 0 {
        //     println!("{}", get_fps());
        // }
        // draw_circle(100., 100., 5., RED);
        // draw_circle(300., 300., 5., BLUE);

        // //TODO why not instead just modify the texture array directly, instead of rendering to a new texture every time
        // create_play_area_texture(&render_target, None, temp);
        // temp += 1;

        // draw_texture_ex(
        //     &render_target.texture,
        //     // &Texture2D::from_rgba8(CANVAS_WIDTH, CANVAS_HEIGHT, &canvas),
        //     BORDER_THICKNESS as f32,
        //     BORDER_THICKNESS as f32,
        //     WHITE,
        //     DrawTextureParams {
        //         flip_y: true,
        //         ..Default::default()
        //     },
        // );
        next_frame().await
    }
}

impl GameCanvas {
    pub fn fill_pixel(&mut self, x: usize, y: usize, color: &Rgba<u8>) {
        self.canvas[(y * CANVAS_WIDTH as usize + x) * 4] = color.r;
        self.canvas[(y * CANVAS_WIDTH as usize + x) * 4 + 1] = color.g;
        self.canvas[(y * CANVAS_WIDTH as usize + x) * 4 + 2] = color.b;
        self.canvas[(y * CANVAS_WIDTH as usize + x) * 4 + 3] = color.a;
    }

    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Rgba<u8>,
    ) {
        for row in y..y + height {
            for col in x..x + width {
                self.fill_pixel(col, row, &color);
                // self.canvas[(row * CANVAS_WIDTH as usize + col) * 4 + 0] = color.r;
                // self.canvas[(row * CANVAS_WIDTH as usize + col) * 4 + 1] = color.g;
                // self.canvas[(row * CANVAS_WIDTH as usize + col) * 4 + 2] = color.b;
                // self.canvas[(row * CANVAS_WIDTH as usize + col) * 4 + 3] = color.a;
            }
        }
    }

    pub fn draw_circle(&mut self, x: usize, y: usize, radius: usize, color: Rgba<u8>) {
        fn is_inside_circle(point: (usize, usize), center: (usize, usize), radius: usize) -> bool {
            fn sq<T: std::ops::Mul<Output = T> + Copy>(x: T) -> T {
                x * x
            }
            sq(point.0 as i32 - center.0 as i32) + sq(point.1 as i32 - center.1 as i32)
                < sq(radius as i32)
        }
        for row in (y - radius)..(y + radius) {
            for col in (x - radius)..(x + radius) {
                // dbg!(row, y, col, x, radius);
                if is_inside_circle((col, row), (x, y), radius) {
                    self.fill_pixel(col, row, &color);
                }
            }
        }
    }

    pub fn rectangle_has_any_color(&self, x: usize, y: usize, width: usize, height: usize) -> bool {
        for row in y..y + height {
            for col in x..x + width {
                for rgbi in 0..3 {
                    if self.canvas[row * CANVAS_WIDTH as usize + col + rgbi] != 0 {
                        return true;
                    }
                }
            }
        }
        return false;
    }

    pub fn at(&self, x: usize, y: usize) -> Rgba<u8> {
        return Rgba::new(
            self.canvas[(y * CANVAS_WIDTH as usize + x) * 4 + 0],
            self.canvas[(y * CANVAS_WIDTH as usize + x) * 4 + 1],
            self.canvas[(y * CANVAS_WIDTH as usize + x) * 4 + 2],
            self.canvas[(y * CANVAS_WIDTH as usize + x) * 4 + 3],
        );
    }

    // pub fn draw_diamond(&mut self, x: usize, y: usize, radius: usize, color: Rgba<u8>) {

    // }
}

fn create_play_area_texture(
    render_target: &RenderTarget,
    camera_reset: Option<&dyn Camera>,
    temp: u32,
) {
    // let texture_camera: Camera2D = Camera2D {
    //     render_target: Some(render_target.clone()),
    //     ..Default::default()
    // };
    let mut texture_camera = Camera2D::from_display_rect(Rect::new(
        0. as f32,
        0.,
        CANVAS_WIDTH as f32,
        CANVAS_HEIGHT as f32,
    ));
    texture_camera.render_target = Some(render_target.clone());

    set_camera(&texture_camera);
    // println!(
    //     "second addr is {:p}",
    //     &texture_camera.render_target.unwrap().texture
    // );
    // clear_background(GREEN);
    // draw_circle(0., 0., 240., RED);
    // draw_circle(100., 100., 143., RED);
    draw_rectangle(
        5. + temp as f32 * get_frame_time() * 60.,
        5.,
        100.,
        200.,
        PINK,
    );
    if temp % 10 == 0 {
        println!("{}", get_fps());
    }
    draw_text("text", 100., 100., 24., BLACK);
    // draw_line(0., 0., 0., CANVAS_HEIGHT_FLOAT, 5.0, BLUE);
    // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);

    // draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, DARKGRAY);

    // set_default_camera();
    // set_camera(&texture_camera);
    // // println!(
    // //     "second addr is {:p}",
    // //     &texture_camera.render_target.unwrap().texture
    // // );
    // clear_background(GREEN);
    // draw_circle(100., 100., 50., RED);

    match camera_reset {
        None => set_default_camera(),
        Some(cam) => set_camera(cam), // TODO untested
    }
    // return texture_camera.render_target.unwrap().clone().texture;
}

fn draw_ui() {
    struct Point {
        x: f32,
        y: f32,
    }
    const BORDER: f32 = BORDER_THICKNESS as f32;
    const WIDTH: f32 = CANVAS_WIDTH as f32;
    const HEIGHT: f32 = CANVAS_HEIGHT as f32;

    let tl = Point {
        x: BORDER / 2.,
        y: BORDER / 2.,
    };
    let tr = Point {
        x: BORDER + WIDTH + BORDER / 2.,
        y: BORDER / 2.,
    };
    let bl = Point {
        x: BORDER / 2.,
        y: BORDER + HEIGHT + BORDER / 2.,
    };
    let br = Point {
        x: BORDER + WIDTH + BORDER / 2.,
        y: BORDER + HEIGHT + BORDER / 2.,
    };

    draw_line(tl.x, tl.y, tr.x, tr.y, BORDER, BORDER_COLOR);
    draw_line(tl.x, tl.y, bl.x, bl.y, BORDER, BORDER_COLOR);
    draw_line(tr.x, tr.y, br.x, br.y, BORDER, BORDER_COLOR);
    draw_line(bl.x, bl.y, br.x, br.y, BORDER, BORDER_COLOR);
}
