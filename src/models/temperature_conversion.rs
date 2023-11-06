use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Celsius {
    pub celsius: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Fahrenheit {
    pub fahrenheit: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Kelvin {
    pub kelvin: f64,
}

impl Celsius {
    pub fn new(celsius: f64) -> Self {
        Celsius { celsius }
    }

    pub fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit {
            fahrenheit: (self.celsius * 1.8) + 32.0,
        }
    }

    pub fn to_kelvin(&self) -> Kelvin {
        Kelvin {
            kelvin: self.celsius + 273.15,
        }
    }
}

impl Fahrenheit {
    pub fn new(fahrenheit: f64) -> Self {
        Fahrenheit { fahrenheit }
    }

    pub fn to_celsius(&self) -> Celsius {
        Celsius {
            celsius: (self.fahrenheit - 32.0) / 1.8,
        }
    }

    pub fn to_kelvin(&self) -> Kelvin {
        Kelvin {
            kelvin: (self.fahrenheit - 32.0) / 1.8 + 273.15,
        }
    }
}

impl Kelvin {
    pub fn new(kelvin: f64) -> Self {
        Kelvin { kelvin }
    }

    pub fn to_celsius(&self) -> Celsius {
        Celsius {
            celsius: self.kelvin - 273.15,
        }
    }

    pub fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit {
            fahrenheit: (self.kelvin - 273.15) * 1.8 + 32.0,
        }
    }
}
