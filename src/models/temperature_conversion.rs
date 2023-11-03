use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Celsius {
    pub celsius: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Fahrenheit {
    pub fahrenheit: f64,
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
}
