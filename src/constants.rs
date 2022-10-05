pub const GRAVITY: f64 = 1.0;
pub const PRESSURE: f64 = 4.0;
pub const VISCOSITY: f64 = 7.0;
pub const CONSOLE_WIDTH: usize = 80;
pub const CONSOLE_HEIGHT: usize = 24;
pub const CHARACTERS: [char; 16] = [
    ' ', '\'', '`', '-', '.', '|', '/', '/', ',', '\\', '|', '\\', '_', '\\', '/', '#',
];

#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub x_pos: f64,
    pub y_pos: f64,
    pub density: f64,
    pub wallflag: i32,
    pub x_force: f64,
    pub y_force: f64,
    pub x_velocity: f64,
    pub y_velocity: f64,
}

pub static mut PARTICLES: [Particle; CONSOLE_WIDTH * CONSOLE_HEIGHT * 2] = [Particle {
    x_pos: 0.0,
    y_pos: 0.0,
    density: 0.0,
    wallflag: 0,
    x_force: 0.0,
    y_force: 0.0,
    x_velocity: 0.0,
    y_velocity: 0.0,
};
    CONSOLE_WIDTH * CONSOLE_HEIGHT * 2];

pub type ScreenBuffer = [u8; CONSOLE_WIDTH * CONSOLE_HEIGHT + 1];
pub static mut SCREEN_BUFFER: ScreenBuffer = [0; CONSOLE_WIDTH * CONSOLE_HEIGHT + 1];

pub static mut VEC_LEN: usize = 0;
pub static mut VEC_CAP: usize = 0;
