use std::io::Error;

use super::terminal::Size;

pub trait UIComponent {
    fn mark_draw(&mut self, value: bool);

    fn need_redraws(&self) -> bool;

    fn resize(&mut self, size: Size) {
        self.set_size(size);
        self.mark_draw(true);
    }

    fn set_size(&mut self, size: Size);

    fn render(&mut self, origin_y: usize) {
        if self.need_redraws() {
            match self.draw(origin_y) {
                Ok(()) => self.mark_draw(false),
                Err(_) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Cannot render the ui component");
                    }
                }
            }
        }
    }

    fn draw(&mut self, origin: usize) -> Result<(), Error>;
}
