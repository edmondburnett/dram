#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Beverage {
    Water = 0,
    Coffee = 1,
    Tea = 2,
}

#[allow(dead_code)]
impl Beverage {
    pub fn code(&self) -> u8 {
        *self as u8
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Beverage::Water),
            1 => Some(Beverage::Coffee),
            2 => Some(Beverage::Tea),
            _ => None,
        }
    }

    // TODO: function to get the effective hydration % of each non-water type
}
