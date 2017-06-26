//! All debug structs and functions go in here

/// Color for debugging
#[derive(Debug, Copy, Clone)]
pub struct DebugColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl DebugColor {
    
    /// Returns a blue color
    pub fn blue()
    -> Self
    {
        Self {
            r: 0,
            g: 0,
            b: 240,
            a: 0,
        }
    }

    /// Returns a red color
    pub fn red()
    -> Self
    {
        Self {
            r: 240,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    /// Returns a red color
    pub fn green()
    -> Self
    {
        Self {
            r: 0,
            g: 240,
            b: 0,
            a: 0,
        }
    }

    /// Returns a yellow color
    pub fn yellow()
    -> Self
    {
        Self {
            r: 255,
            g: 255,
            b: 0,
            a: 0,
        }
    }
}