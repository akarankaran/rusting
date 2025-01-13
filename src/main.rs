use std::fmt;

struct Temperature {
    celsius: f64,
    fahrenheit: f64,
}

impl Temperature {
    fn new_celsius(celsius: f64) -> Self {
        Temperature {
            celsius,
            fahrenheit: celsius * 9.0 / 5.0 + 32.0,
        }
    }

    fn new_fahrenheit(fahrenheit: f64) -> Self {
        Temperature {
            fahrenheit,
            celsius: (fahrenheit - 32.0) * 5.0 / 9.0,
        }
    }

    fn to_celsius(&self) -> f64 {
        self.celsius
    }

    fn to_fahrenheit(&self) -> f64 {
        self.fahrenheit
    }
}

impl fmt::Display for Temperature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:.2} °C / {:.2} °F", self.celsius, self.fahrenheit)
    }
}

fn main() {
    let temp1 = Temperature::new_celsius(25.0);
    println!("{}", temp1);
    
    let temp2 = Temperature::new_fahrenheit(77.0);
    println!("{}", temp2);
    
    let temp3 = Temperature::new_celsius(-10.0);
    println!("Celsius: {}, Fahrenheit: {}", temp3.to_celsius(), temp3.to_fahrenheit());
    
    let temp4 = Temperature::new_fahrenheit(32.0);
    println!("Celsius: {}, Fahrenheit: {}", temp4.to_celsius(), temp4.to_fahrenheit());
    
    let temp5 = Temperature::new_celsius(100.0);
    println!("Temperature in Fahrenheit: {}", temp5.to_fahrenheit());
    
    let temp6 = Temperature::new_fahrenheit(212.0);
    println!("Temperature in Celsius: {}", temp6.to_celsius());
}