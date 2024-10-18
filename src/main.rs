use std::f32::consts::PI;

use macroquad::prelude::*;

const CANVAS_WIDTH: u16 = 480;
const CANVAS_HEIGHT: u16 = 720;
const BORDER_THICKNESS: u16 = 5;
const BORDER_COLOR: Color = GRAY;

const TURN_SPEED: f32 = PI / 128.;
const DEFAULT_LINE_SPEED: f32 = 30.;
const PLAYER_RADIUS: usize = 5;

#[derive(Debug)]
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

#[derive(Debug)]
struct GameCanvas {
    canvas: Vec<u8>,
}

#[derive(Debug)]
struct Direction {
    angle: f32,
}

impl Direction {
    pub fn from_angle(angle: f32) -> Self {
        Self { angle: -angle }
    }
    pub fn from_xy(x: f32, y: f32) -> Self {
        // x = sin(angle), y = cos(angle)
        Self { angle: x.atan2(y) }
    }
    pub fn xy(&self) -> (f32, f32) {
        self.angle.sin_cos()
    }
    pub fn x(&self) -> f32 {
        self.angle.sin()
    }
    pub fn y(&self) -> f32 {
        self.angle.cos()
    }
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
#[derive(Debug)]
struct PlayerControls {
    left: KeyCode,
    right: KeyCode,
}

#[derive(Debug)]
struct Player {
    location: Point<f32>,
    direction: Direction,
    controls: PlayerControls,
    color: Rgba<u8>,
    name: String,
}
impl Player {
    pub fn coords(&self) -> Point<usize> {
        Point {
            x: self.location.x as usize,
            y: self.location.y as usize,
        }
    }

    pub fn colliders(&self) -> Vec<Point<usize>> {
        let mut colliders: Vec<Point<usize>> = Vec::new();
        colliders.push(Point {
            x: (self.location.x + self.direction.x() * PLAYER_RADIUS as f32 * (1.2)) as usize,
            y: (self.location.y + self.direction.y() * PLAYER_RADIUS as f32 * (1.2)) as usize,
        });
        colliders
    }

    pub fn check_collision(&self, canvas: &GameCanvas) -> bool {
        let colliders = self.colliders();
        return colliders
            .iter()
            .any(|collider| !canvas.at(&collider).alphaless_match(&Rgba::new(0, 0, 0, 0)));
    }
}

#[macroquad::main("Curve Game")]
async fn main() {
    let mut canvas = GameCanvas {
        canvas: (0..CANVAS_HEIGHT as usize * CANVAS_WIDTH as usize)
            .flat_map(|_| [0, 0, 0, 255])
            .collect(),
    };

    // let mut direction_angle = 0.0f32;
    // let mut location = Vec2::new(100., 100.);
    let mut line_speed_multiplier = 1.;
    let mut players: Vec<Player> = Vec::with_capacity(8);
    players.push(Player {
        location: Point { x: 100., y: 100. },
        direction: Direction::from_xy(0., 1.),
        controls: PlayerControls {
            left: KeyCode::A,
            right: KeyCode::D,
        },
        color: Rgba::new(255, 0, 0, 255),
        name: String::from("Red"),
    });
    players.push(Player {
        location: Point {
            x: CANVAS_WIDTH as f32 - 100.,
            y: CANVAS_HEIGHT as f32 - 100.,
        },
        direction: Direction::from_xy(0., -1.),
        controls: PlayerControls {
            left: KeyCode::Left,
            right: KeyCode::Right,
        },
        color: Rgba::new(0, 0, 255, 255),
        name: String::from("Blue"),
    });
    let mut end_game = false;
    loop {
        if is_key_down(KeyCode::Escape) {
            return;
        }
        clear_background(WHITE);
        draw_ui();

        if !end_game {
            for player in &mut players {
                if is_key_down(player.controls.left) {
                    player.direction.angle += TURN_SPEED % (2. * PI);
                }
                if is_key_down(player.controls.right) {
                    player.direction.angle -= TURN_SPEED % (2. * PI);
                }

                player.location.x += player.direction.x()
                    * line_speed_multiplier
                    * DEFAULT_LINE_SPEED
                    * get_frame_time();
                player.location.y += player.direction.y()
                    * line_speed_multiplier
                    * DEFAULT_LINE_SPEED
                    * get_frame_time();

                if player.check_collision(&canvas) {
                    end_game = true;
                }

                canvas.draw_circle(
                    player.location.x,
                    player.location.y,
                    PLAYER_RADIUS as f32,
                    &player.color,
                );
            }
        }

        draw_texture(
            &Texture2D::from_rgba8(CANVAS_WIDTH, CANVAS_HEIGHT, &canvas.canvas),
            BORDER_THICKNESS as f32,
            BORDER_THICKNESS as f32,
            LIGHTGRAY,
        );
        for player in [&players[1], &players[0]] {
            // draw head
            draw_circle(
                player.location.x + BORDER_THICKNESS as f32,
                player.location.y + BORDER_THICKNESS as f32,
                PLAYER_RADIUS as f32,
                LIGHTGRAY,
            );

            //draw collision tip
            // for tip in player.colliders() {
            //     draw_circle(
            //         tip.x as f32 + BORDER_THICKNESS as f32,
            //         tip.y as f32 + BORDER_THICKNESS as f32,
            //         2.,
            //         LIME,
            //     );
            // }
        }
        if end_game {
            line_speed_multiplier = 0.;
            draw_text("Owie :(", 200., 200., 40., WHITE);
        };
        next_frame().await;
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
            }
        }
    }

    pub fn draw_circle(&mut self, x: f32, y: f32, radius: f32, color: &Rgba<u8>) {
        fn is_inside_circle(point: (usize, usize), center: (f32, f32), radius: f32) -> bool {
            fn sq<T: std::ops::Mul<Output = T> + Copy>(num: T) -> T {
                num * num
            }
            sq(point.0 as i32 - center.0 as i32) + sq(point.1 as i32 - center.1 as i32)
                < sq(radius as i32)
        }
        fn positive_round(num: f32) -> usize {
            (num + 0.5) as usize
        }
        for row in (positive_round(y - radius))..(positive_round(y + radius)) {
            for col in (positive_round(x - radius))..(positive_round(x + radius)) {
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

    pub fn at(&self, loc: &Point<usize>) -> Rgba<u8> {
        return Rgba::new(
            self.canvas[(loc.y * CANVAS_WIDTH as usize + loc.x) * 4 + 0],
            self.canvas[(loc.y * CANVAS_WIDTH as usize + loc.x) * 4 + 1],
            self.canvas[(loc.y * CANVAS_WIDTH as usize + loc.x) * 4 + 2],
            self.canvas[(loc.y * CANVAS_WIDTH as usize + loc.x) * 4 + 3],
        );
    }
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
