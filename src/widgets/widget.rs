use std::io::Write;

pub trait Widget {
    /// draw the widget via TTY
    fn draw<W: Write>(&mut self, stdout: &mut W);
}
