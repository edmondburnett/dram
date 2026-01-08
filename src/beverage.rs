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
        // usage: let code: u8 = Beverage::Coffee.code();
        *self as u8
    }

    pub fn string(&self) -> &str {
        // let message = format!("I'm drinking {}", beverage.string());
        match self {
            Beverage::Water => "water",
            Beverage::Coffee => "coffee",
            Beverage::Tea => "tea",
        }
    }

    pub fn from_code(code: u8) -> Option<Self> {
        match code {
            0 => Some(Beverage::Water),
            1 => Some(Beverage::Coffee),
            2 => Some(Beverage::Tea),
            _ => None,
        }
    }
}
