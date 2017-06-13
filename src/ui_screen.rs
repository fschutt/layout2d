/// UI screen 
#[derive(Debug)]
pub struct UiScreen {
    pub min_width: u32,
    pub min_height: u32,
}

impl UiScreen {

    /// Creates a new UiScreen
    #[inline]
    pub fn new() -> Self {
        Self {
            min_width: 600,
            min_height: 400,
        }
    }

    /// Refreshes the UiScreen, returns if the frame has to be redrawn or not
    #[inline]
    pub(crate) fn layout(&mut self)
    -> bool 
    {

        true
    }
}