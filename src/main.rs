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
    header: &'static HeaderColorConfig,
    footer: &'static FooterColorConfig,
}

struct State<'a> {
    color_config: ColorConfig,
    container: Container,
    header_text: String,
    input_text: &'a mut String,
}

fn main() {
    let stdin = stdin();
    let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());

    let (width, height) = termion::terminal_size().unwrap();
    let state = State {
        color_config: ColorConfig {
            header: &HeaderColorConfig {
                fg: (255, 255, 255),
                bg: (68, 71, 90),
            },
            footer: &FooterColorConfig {
                fg: (255, 255, 255),
                bg: (68, 71, 90),
            },
        },
        container: Container {
            x: 1,
            y: 1,
            width: width,
            height,
        },
        header_text: String::from("hello world"),
        input_text: &mut String::from(""),
    };

    draw_ui(&state, &mut stdout);

    for c in stdin.events() {
        let event = c.unwrap();

        let mut inputted_command = None;

        match event {
            Event::Key(key) => match key {
                Key::Ctrl('c') => break,
                Key::Char('\n') => {
                    inputted_command = command::parse_input(&state.input_text);
                    state.input_text.clear();
                }
                Key::Delete | Key::Backspace => {
                    state.input_text.pop();
                }
                Key::Char(key) => {
                    state.input_text.push(key);
                }
                _ => {}
            },
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, a, b) => write!(stdout, "{}", cursor::Goto(a, b)).unwrap(),
                MouseEvent::Release(a, b) => write!(stdout, "{}", cursor::Goto(a, b)).unwrap(),
                MouseEvent::Hold(a, b) => write!(stdout, "{}", cursor::Goto(a, b)).unwrap(),
            },
            Event::Unsupported(_) => {}
        }

        draw_ui(&state, &mut stdout);

        match inputted_command {
            Some(command::CommandKind::Quit) => break,
            _ => {}
        }
    }
    draw::clear_screen(&mut stdout);
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn draw_ui<W: Write>(state: &State, stdout: &mut W) {
    let header = Header {
        container: &state.container,
        color_config: state.color_config.header,
        display_text: &state.header_text,
    };

    let footer = Footer {
        container: &state.container,
        color_config: state.color_config.footer,
        input_text: state.input_text,
    };

    draw::clear_screen(stdout);
    header.draw(stdout);
    footer.draw(stdout);
    stdout.flush().unwrap();
}
