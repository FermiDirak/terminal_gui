use super::super::Write;

pub trait Widget {
    /// draw the widget via tty
    fn draw(&self, stdout: &mut Write);
}
