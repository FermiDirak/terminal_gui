use std::convert::TryFrom;
use std::io::Write;

use super::super::utils::{draw, Container};
use super::widget::Widget;

#[derive(Debug)]
pub struct Header<'a> {
    pub container: &'a Container,
    pub display_text: String,
}

impl<'a> Widget for Header<'a> {
    fn draw<W: Write>(&self, stdout: &mut W) {
        let header_area = Container {
            x: self.container.x,
            y: self.container.y,
            width: self.container.width,
            height: 1,
        };

        let text_offset = u16::try_from(self.display_text.len() / 2).unwrap();
        let text_x_start = self.container.x + (self.container.width / 2) - text_offset;

        draw::fill_area(stdout, &header_area);
        draw::write_text(stdout, &self.display_text, (text_x_start, header_area.y));
    }
}
