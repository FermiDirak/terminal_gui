use std::io::{stdin, stdout, Write};

use termion::cursor;
use termion::event::Key;
use termion::event::{Event, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

mod command;
mod utils;
mod widgets;

use utils::{draw, Container};
use widgets::{Footer, FooterColorConfig, Header, HeaderColorConfig, Widget};

struct ColorConfig {
    header: HeaderColorConfig,
    footer: FooterColorConfig,
}

struct State<'a> {
    color_config: &'a ColorConfig,
    container: &'a Container,
    header_text: String,
    input_text: String,
}

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    draw::clear_screen(&mut stdout);

    let (width, height) = termion::terminal_size().unwrap();

    let state = State {
        color_config: &ColorConfig {
            header: HeaderColorConfig {
                fg: (255, 255, 255),
                bg: (68, 71, 90),
            },
            footer: FooterColorConfig {
                fg: (255, 255, 255),
                bg: (68, 71, 90),
            },
        },
        container: &Container {
            x: 1,
            y: 1,
            width: width,
            height,
        },
        header_text: String::from("hello world"),
        input_text: String::from(""),
    };

    let mut header = Header {
        container: state.container,
        color_config: &state.color_config.header,
        display_text: state.header_text,
    };

    let mut footer = Footer {
        container: state.container,
        color_config: &state.color_config.footer,
        input_text: state.input_text,
    };

    header.draw(&mut stdout);
    footer.draw(&mut stdout);

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
    draw::clear_screen(&mut stdout);
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
