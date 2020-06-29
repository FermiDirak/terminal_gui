use std::io::{stdin, stdout, Write};

// use termion::{color, clear, cursor};
use termion::cursor;
use termion::event::Key;
use termion::event::{Event, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

mod utils;

use utils::{draw, Container};

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    draw::clear_screen(&mut stdout);

    let (width, height) = termion::terminal_size().unwrap();
    let container = Container {
        x: 1,
        y: 1,
        width: width,
        height,
    };

    draw::fill_area(&mut stdout, container);

    stdout.flush().unwrap();

    for c in stdin.events() {
        let event = c.unwrap();

        match event {
            Event::Key(key) => match key {
                Key::Char('q') => break,
                Key::Char(key) => write!(stdout, "{}", key).unwrap(),
                Key::Ctrl(key) => write!(stdout, "Ctrl-{}", key).unwrap(),
                Key::Left => write!(stdout, "<left>").unwrap(),
                _ => {}
            },
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, a, b) => write!(stdout, "{}", cursor::Goto(a, b)).unwrap(),
                MouseEvent::Release(a, b) => write!(stdout, "{}", cursor::Goto(a, b)).unwrap(),
                MouseEvent::Hold(a, b) => write!(stdout, "{}", cursor::Goto(a, b)).unwrap(),
            },
            Event::Unsupported(_) => {}
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
