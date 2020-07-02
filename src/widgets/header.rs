use std::io::Write;

use super::super::utils::{draw, Container};
use super::widget::Widget;

#[derive(Debug)]
pub struct Header<'a> {
    pub container: &'a Container,
    pub displayText: String,
}

impl<'a> Widget for Header<'a> {
    fn draw<W: Write>(&mut self, stdout: &mut W) {
        let header_area = Container {
            x: self.container.x,
            y: self.container.y,
            width: self.container.width,
            height: 1,
        };
        draw::fill_area(stdout, &header_area);
    }
}
