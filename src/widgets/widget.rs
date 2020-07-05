use super::super::tgui::Write;

pub trait Widget {
    /// draw the widget via TTY
    fn draw(&self, stdout: &mut Write);
}
