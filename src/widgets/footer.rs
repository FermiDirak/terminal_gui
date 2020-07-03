use std::io::Write;

use super::super::utils::{draw, Container};
use super::widget::Widget;

#[derive(Debug)]
pub struct FooterColorConfig {
    pub fg: draw::Color,
    pub bg: draw::Color,
}

pub struct Footer<'a> {
    pub container: Container,
    pub color_config: &'a FooterColorConfig,
    pub input_text: String,
}

impl<'a> Footer<'a> {
    pub fn update(&mut self, input_text: String) {
        self.input_text = input_text;
    }
}

impl<'a, W: Write> Widget<W> for Footer<'a> {
    fn draw(&self, stdout: &mut W) {
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
            input_text.clone(),
            (1, footer_area.y),
            color_config.fg,
            color_config.bg,
        );
    }
}
