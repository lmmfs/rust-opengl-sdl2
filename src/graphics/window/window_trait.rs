pub struct WindowData<'a> {
    pub name: &'a str,
    pub width: usize, 
    pub height: usize,
}

pub trait Window {
    fn create(window_data: WindowData) -> Result<Self, String> where Self: Sized;
    fn get_size(&self) -> (u32, u32);
    //fn poll_events(&self) -> Vec<crate::event::Event>; // Assuming you have an Event type
    fn swap_buffers(&self);
    // ... other window-related methods
}

