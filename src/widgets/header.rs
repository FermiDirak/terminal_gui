use std::io::Write;

use termion::event::Event;

use super::super::utils::{draw, Container};
use super::widget::Widget;

#[derive(Debug)]
pub struct Header<W: Write> {
    pub container: Container,
    pub displayText: String,
    pub stdout: W,
}

impl<W: Write> Widget for Header<W> {
    fn draw(&mut self, event: Event) {
        let header_area = Container {
            x: self.container.x,
            y: self.container.y,
            width: self.container.width,
            height: 1,
        };
        draw::fill_area(&mut self.stdout, header_area);
    }
}
