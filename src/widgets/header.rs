use std::convert::TryFrom;

use super::super::tgui::widget::Widget;
use super::super::tgui::{draw, Container, Write};

#[derive(Debug)]
pub struct HeaderColorConfig {
    pub fg: draw::Color,
    pub bg: draw::Color,
}

#[derive(Debug)]
pub struct Header<'a> {
    pub container: &'a Container,
    pub color_config: &'a HeaderColorConfig,
    pub display_text: &'a str,
}

impl<'a> Widget for Header<'a> {
    fn draw(&self, stdout: &mut Write) {
        let Header {
            container,
            color_config,
            display_text,
        } = self;
        let header_area = Container {
            x: container.x,
            y: container.y,
            width: container.width,
            height: 1,
        };

        let text_offset = u16::try_from(display_text.len() / 2).unwrap();
        let text_x_start = container.x + (container.width / 2) - text_offset;

        draw::fill_area(stdout, &header_area, color_config.bg);
        draw::write_text(
            stdout,
            display_text.to_string(),
            (text_x_start, header_area.y),
            color_config.fg,
            color_config.bg,
        );
    }
}
