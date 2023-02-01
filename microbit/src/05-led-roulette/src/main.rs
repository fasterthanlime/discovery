#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    self as _,
    display::blocking::Display,
    hal::{prelude::*, timer::Timer},
    Board,
};
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

#[derive(Clone, Copy)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn next(self) -> Self {
        match self {
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
            Dir::Up => Dir::Right,
        }
    }

    fn edge(self, x: u32, y: u32) -> bool {
        match self {
            Dir::Right => x == 4,
            Dir::Down => y == 4,
            Dir::Left => x == 0,
            Dir::Up => y == 0,
        }
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Up => (0, -1),
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut x: u32 = 0;
    let mut y: u32 = 0;
    let mut dir = Dir::Right;

    loop {
        let mut matrix = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ];
        matrix[y as usize][x as usize] = 1;

        let (dx, dy) = dir.delta();
        x = x.checked_add_signed(dx).unwrap();
        y = y.checked_add_signed(dy).unwrap();

        if dir.edge(x, y) {
            dir = dir.next();
        }

        display.show(&mut timer, matrix, 16_u32);
    }
}
