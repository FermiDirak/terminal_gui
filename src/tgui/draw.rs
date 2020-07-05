use std::io::Write as _;
use termion::{clear, color, cursor};

use super::Container;
use super::Write;

pub type Color = (u8, u8, u8);

pub fn create_termion_rgb(color: Color) -> termion::color::Rgb {
    let (r, g, b) = color;
    return color::Rgb(r, g, b);
}

pub fn clear_screen(stdout: &mut Write) {
    write!(
        stdout,
        "{}{}{}",
        clear::All,
        cursor::Goto(1, 1),
        cursor::Hide,
    )
    .unwrap();
}

pub fn fill_area(stdout: &mut Write, container: &Container, bg: Color) {
    let Container {
        x,
        y,
        width,
        height,
    } = *container;
    let spaces = " ".repeat(width.into());

    for y_curr in y..y + height {
        write!(
            stdout,
            "{}{}{}{}",
            cursor::Goto(x, y_curr),
            color::Bg(create_termion_rgb(bg)),
            spaces,
            color::Bg(color::Reset),
        )
        .unwrap();
    }
}

pub fn write_text(stdout: &mut Write, text: String, pos: (u16, u16), fg: Color, bg: Color) {
    let (x, y) = pos;
    write!(
        stdout,
        "{}{}{}{}{}{}",
        cursor::Goto(x, y),
        color::Fg(create_termion_rgb(fg)),
        color::Bg(create_termion_rgb(bg)),
        text,
        color::Fg(color::Reset),
        color::Bg(color::Reset),
    )
    .unwrap();
}
