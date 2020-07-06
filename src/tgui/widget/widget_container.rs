use std::io::Write as _;

use super::super::draw;
use super::super::widget::Widget;
use super::super::Write;

pub struct WidgetContainer<'a> {
    widgets: Vec<Box<&'a dyn Widget>>,
}

impl<'a> Default for WidgetContainer<'a> {
    fn default() -> WidgetContainer<'a> {
        WidgetContainer {
            widgets: Vec::new(),
        }
    }
}

impl<'a> WidgetContainer<'a> {
    pub fn register(&mut self, widget: Box<&'a dyn Widget>) {
        self.widgets.push(widget);
    }

    pub fn draw(&self, stdout: &mut Write) {
        draw::clear_screen(stdout);
        for widget in self.widgets.iter() {
            widget.draw(stdout);
        }
        stdout.flush().unwrap();
    }
}
