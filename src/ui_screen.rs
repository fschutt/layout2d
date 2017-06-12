//! Definition for UI screen
pub struct UiScreen {

}

impl UiScreen {

    /// Creates a new UiScreen
    #[inline]
    pub fn new() -> Self {
        Self {

        }
    }

    /// Refreshes the UiScreen, returns if the frame has to be redrawn or not
    #[inline]
    pub(crate) fn layout(&mut self)
    -> bool 
    {
        // todo
        true
    }
}