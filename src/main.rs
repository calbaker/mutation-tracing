use std::fmt;

// A simple TrackedState implementation
#[derive(Debug)]
struct TrackedState<Q>
where
    Q: Clone + fmt::Debug,
{
    value: Option<Q>,
    update_location: Option<String>,
    name: String,
}

impl<Q> TrackedState<Q>
where
    Q: Clone + fmt::Debug,
{
    fn new(name: &str) -> Self {
        Self {
            value: None,
            update_location: None,
            name: name.to_string(),
        }
    }

    fn update(&mut self, value: Q) {
        self.value = Some(value);
        self.update_location = Some(format!("{}:{}", file!(), line!()));
    }

    fn reset(&mut self) {
        self.value = None;
        self.update_location = None;
    }

    fn assert_updated(&self) {
        assert!(
            self.value.is_some(),
            "State variable '{}' was not updated!",
            self.name
        );
    }

    fn get(&self) -> Option<&Q> {
        self.value.as_ref()
    }

    fn get_with_location(&self) -> Option<(&Q, &str)> {
        match (&self.value, &self.update_location) {
            (Some(val), Some(loc)) => Some((val, loc)),
            _ => None,
        }
    }

    fn map<F, R>(&self, f: F) -> Option<R>
    where
        F: FnOnce(&Q) -> R,
    {
        self.value.as_ref().map(f)
    }

    fn map_mut<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Q),
    {
        if let Some(ref mut val) = self.value {
            f(val);
            // Update location since the value was potentially modified
            self.update_location = Some(format!("{}:{}", file!(), line!()));
        }
    }
}

// Import uom for demonstration
use uom::si::f64::*;
use uom::si::length::meter;
use uom::si::power::watt;
use uom::si::thermodynamic_temperature::kelvin;

// A sample powertrain component using tracked states
struct Battery {
    temperature: TrackedState<ThermodynamicTemperature>,
    power_output: TrackedState<Power>,
    dimensions: TrackedState<Length>,
}

impl Battery {
    fn new() -> Self {
        Self {
            temperature: TrackedState::new("battery_temperature"),
            power_output: TrackedState::new("battery_power_output"),
            dimensions: TrackedState::new("battery_dimensions"),
        }
    }

    fn update_state(&mut self, ambient_temp: ThermodynamicTemperature, power_draw: Power) {
        // Update temperature based on some calculation
        self.temperature
            .update(ThermodynamicTemperature::new::<kelvin>(
                300.0 + power_draw.get::<watt>() * 0.01,
            ));

        // Update power output
        self.power_output.update(power_draw);

        // Note: We're deliberately not updating dimensions to show error detection
    }

    fn reset_tracked_states(&mut self) {
        self.temperature.reset();
        self.power_output.reset();
        self.dimensions.reset();
    }

    fn assert_all_updated(&self) {
        self.temperature.assert_updated();
        self.power_output.assert_updated();
        self.dimensions.assert_updated();
    }

    fn print_debug_info(&self) {
        println!("Battery state:");
        if let Some((temp, loc)) = self.temperature.get_with_location() {
            println!("  Temperature: {:?} (updated at {})", temp, loc);
        } else {
            println!("  Temperature: Not updated");
        }

        if let Some((power, loc)) = self.power_output.get_with_location() {
            println!("  Power: {:?} (updated at {})", power, loc);
        } else {
            println!("  Power: Not updated");
        }

        if let Some((dims, loc)) = self.dimensions.get_with_location() {
            println!("  Dimensions: {:?} (updated at {})", dims, loc);
        } else {
            println!("  Dimensions: Not updated");
        }
    }
}

fn main() {
    // Create a battery
    let mut battery = Battery::new();

    // Set initial dimensions just once (doesn't change per time step)
    battery.dimensions.update(Length::new::<meter>(0.5));

    // Simulation loop
    for step in 0..3 {
        println!("\nTime step {}", step);

        // Reset tracked states at the beginning of each iteration
        battery.reset_tracked_states();

        // Update the battery state
        let ambient_temp = ThermodynamicTemperature::new::<kelvin>(298.0);
        let power_draw = Power::new::<watt>(1000.0 * (step as f64 + 1.0));
        battery.update_state(ambient_temp, power_draw);

        // Print debug info
        battery.print_debug_info();

        // Verify all states were updated
        println!("Checking if all states were updated...");

        // This will panic because dimensions wasn't updated in update_state()
        // Uncomment to see the assertion failure:
        // battery.assert_all_updated();

        // Instead, let's check individual fields
        battery.temperature.assert_updated();
        battery.power_output.assert_updated();

        // Fix the dimensions manually for this step
        if battery.dimensions.get().is_none() {
            println!("Warning: Battery dimensions not updated in this step");
            battery.dimensions.update(Length::new::<meter>(0.5));
        }
    }

    // Example of using map and map_mut
    if let Some(temp) = battery.temperature.get() {
        println!("\nCurrent battery temperature: {:?}", temp);
    }

    // Use map to transform the value
    let temp_celsius = battery
        .temperature
        .map(|temp| temp.get::<kelvin>() - 273.15);
    println!("Temperature in Celsius: {:.2}°C", temp_celsius.unwrap());

    // Use map_mut to modify the value in-place
    battery.temperature.map_mut(|temp| {
        *temp = ThermodynamicTemperature::new::<kelvin>(temp.get::<kelvin>() + 5.0);
    });

    println!(
        "Modified temperature: {:?}",
        battery.temperature.get().unwrap()
    );
    println!("Last updated at: {:?}", battery.temperature.update_location);
}
