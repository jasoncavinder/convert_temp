use std::io::stdin;
fn main() {
    println!("Convert temperatures between Fahrenheit, Celsius, and Kelvin.");
    loop {
        println!("Enter a temperature and a unit (F, C, or K) to convert to the other units.");
        let mut input: String = String::new();

        stdin().read_line(&mut input).expect("Failed to read line");

        if input.trim().is_empty() {
            println!("Exiting.");
            break;
        }

        let input_temperature: Temperature = match Temperature::from(input) {
            Ok(temp) => temp,
            Err(err) => {
                println!("{}\n", err);
                continue;
            }
        };

        println!(
            "{} is {} and {}.",
            input_temperature.to_string(3),
            match input_temperature.unit {
                TemperatureUnits::Fahrenheit => input_temperature.as_celcius().to_string(3),
                TemperatureUnits::Celsius => input_temperature.as_fahrenheit().to_string(3),
                TemperatureUnits::Kelvin => input_temperature.as_celcius().to_string(3),
            },
            match input_temperature.unit {
                TemperatureUnits::Fahrenheit => input_temperature.as_kelvin().to_string(3),
                TemperatureUnits::Celsius => input_temperature.as_kelvin().to_string(3),
                TemperatureUnits::Kelvin => input_temperature.as_fahrenheit().to_string(3),
            }
        );
        println!("\nEnter another temperature to convert or press enter to exit.");
    }
}

#[derive(PartialEq, PartialOrd)]
enum TemperatureUnits {
    Fahrenheit,
    Celsius,
    Kelvin,
}
impl TemperatureUnits {
    fn to_string(&self) -> String {
        match self {
            TemperatureUnits::Fahrenheit => "Fahrenheit",
            TemperatureUnits::Celsius => "Celsius",
            TemperatureUnits::Kelvin => "Kelvin",
        }
        .to_string()
    }
}

#[derive(PartialEq, PartialOrd)]
struct Temperature {
    temperature: f64,
    unit: TemperatureUnits,
}
impl Temperature {
    fn from(mut input: String) -> Result<Temperature, &'static str> {
        input = input.trim().to_string();

        let unit = match input
            .pop()
            .unwrap_or_default()
            .to_uppercase()
            .to_string()
            .as_str()
        {
            "F" => TemperatureUnits::Fahrenheit,
            "C" => TemperatureUnits::Celsius,
            "K" => TemperatureUnits::Kelvin,
            _ => {
                return Err("Invalid unit.");
            }
        };

        let temperature_value: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                return Err("Invalid temperature.");
            }
        };
        let temperature: Temperature = Temperature {
            temperature: temperature_value,
            unit: unit,
        };
        let absolute_zero: Temperature = Temperature {
            temperature: 0.0,
            unit: TemperatureUnits::Kelvin,
        };

        if temperature.as_kelvin() < absolute_zero {
            return Err("Temperature is below absolute zero.");
        }

        Ok(temperature)
    }

    fn as_fahrenheit(&self) -> Temperature {
        Temperature {
            temperature: match self.unit {
                TemperatureUnits::Fahrenheit => self.temperature,
                TemperatureUnits::Celsius => self.temperature * 9.0 / 5.0 + 32.0,
                TemperatureUnits::Kelvin => self.temperature * 9.0 / 5.0 - 459.67,
            },
            unit: TemperatureUnits::Fahrenheit,
        }
    }

    fn as_celcius(&self) -> Temperature {
        Temperature {
            temperature: match self.unit {
                TemperatureUnits::Fahrenheit => (self.temperature - 32.0) * 5.0 / 9.0,
                TemperatureUnits::Celsius => self.temperature,
                TemperatureUnits::Kelvin => self.temperature - 273.15,
            },
            unit: TemperatureUnits::Celsius,
        }
    }

    fn as_kelvin(&self) -> Temperature {
        Temperature {
            temperature: match self.unit {
                TemperatureUnits::Fahrenheit => (self.temperature + 459.67) * 5.0 / 9.0,
                TemperatureUnits::Celsius => self.temperature + 273.15,
                TemperatureUnits::Kelvin => self.temperature,
            },
            unit: TemperatureUnits::Kelvin,
        }
    }

    fn to_string(&self, precision: i8) -> String {
        let multipler = 10.0_f64.powi(precision.into());
        format!(
            "{:?} {}{}",
            (self.temperature * multipler).round() / multipler,
            match self.unit {
                TemperatureUnits::Kelvin => "",
                _ => "\u{00B0}",
            },
            self.unit.to_string()
        )
    }
}
