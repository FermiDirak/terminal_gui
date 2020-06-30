use termion::event::Event;

pub trait Widget {
    /// draw the widget via TTY
    fn draw(&mut self, event: Event);
}
