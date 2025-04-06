use crate::graphics::window::window_trait::Window;

pub struct Application<W: Window> {
    window: W
}