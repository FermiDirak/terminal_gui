use std::io::{stdin, stdout, Stdin, Stdout};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::{IntoRawMode, RawTerminal};

use super::draw;
use super::Container;

pub type Write = MouseTerminal<RawTerminal<Stdout>>;

pub struct Terminal<'a> {
    pub stdout: Write,
    pub container: Container,
    pub draw: &'a dyn Fn(termion::event::Event),
}

impl<'a> Terminal<'a> {
    pub fn run(&self) {
        for event in stdin().events() {
            let event = event.unwrap();

            (self.draw)(event);
        }

        // draw::clear_screen(&mut self.stdout);
        // write!(stdout, "{}", termion::cursor::Show).unwrap();
    }
}

pub struct TerminalBuilder<'a> {
    pub draw: &'a dyn Fn(termion::event::Event),
}

impl<'a> TerminalBuilder<'a> {
    pub fn new() -> Self {
        TerminalBuilder { draw: &|_| {} }
    }

    pub fn draw(&mut self, draw_fn: &'a dyn Fn(termion::event::Event)) -> &mut Self {
        self.draw = draw_fn;
        self
    }

    pub fn execute(&self) -> Terminal {
        let (width, height) = termion::terminal_size().unwrap();

        Terminal {
            stdout: MouseTerminal::from(stdout().into_raw_mode().unwrap()),
            container: Container {
                x: 1,
                y: 1,
                width,
                height,
            },
            draw: self.draw,
        }
    }
}
