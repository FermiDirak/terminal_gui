use std::io::Write;

use super::super::utils::{draw, Container};
use super::widget::Widget;

#[derive(Debug)]
pub struct FooterColorConfig {
    pub fg: draw::Color,
    pub bg: draw::Color,
}

#[derive(Debug)]
pub struct Footer<'a> {
    pub container: &'a Container,
    pub color_config: &'a FooterColorConfig,
    pub input_text: String,
}

impl<'a> Widget for Footer<'a> {
    fn draw<W: Write>(&self, stdout: &mut W) {
        let Footer {
            container,
            color_config,
            input_text,
        } = self;
        let footer_area = Container {
            x: container.x,
            y: container.y + container.height - 1,
            width: container.width,
            height: 1,
        };

        draw::fill_area(stdout, &footer_area, color_config.bg);
        draw::write_text(
            stdout,
            &input_text,
            (1, footer_area.y),
            color_config.fg,
            color_config.bg,
        );
    }
}
