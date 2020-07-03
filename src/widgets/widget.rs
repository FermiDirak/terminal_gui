use std::io::Write;

pub trait Widget<W: Write> {
    /// draw the widget via TTY
    fn draw(&self, stdout: &mut W);
}
