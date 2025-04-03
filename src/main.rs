use std::fmt;

// A simple TrackedState implementation
#[derive(Debug)]
struct TrackedState<Q>
where
    Q: fmt::Debug,
{
    value: Option<Q>,
    update_location: Option<String>,
    name: String,
}

impl<Q> TrackedState<Q>
where
    Q: fmt::Debug,
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
}

// Import uom for demonstration
use uom::si::f64::*;
use uom::si::heat_capacity::joule_per_kelvin;
use uom::si::power::watt;
use uom::si::thermodynamic_temperature::kelvin;
use uom::si::time::second;

// A minimal battery component
struct Battery {
    temperature: TrackedState<ThermodynamicTemperature>,
    power_output: TrackedState<Power>,
}

impl Battery {
    fn new() -> Self {
        Self {
            temperature: TrackedState::new("battery_temperature"),
            power_output: TrackedState::new("battery_power_output"),
        }
    }

    fn update_state(&mut self, power_draw: Power, time_step: Time) {
        // Update power output
        self.power_output.update(power_draw);

        // Properly calculate temperature change using dimensional analysis
        let temp_if_none = ThermodynamicTemperature::new::<kelvin>(298.0);
        let current_temp = self.temperature.get().unwrap_or(&temp_if_none);

        // Convert power and time to energy (joules)
        let energy: Energy = power_draw * time_step;

        // Simple thermal model: 1 joule raises temp by 0.001 K
        // (This would normally depend on mass and specific heat capacity)
        let temp_change: TemperatureInterval = energy / HeatCapacity::new::<joule_per_kelvin>(1.0);

        // Apply the temperature change using proper unit math
        let new_temp = *current_temp + temp_change;

        // Update temperature
        self.temperature.update(new_temp);
    }

    fn reset_tracked_states(&mut self) {
        self.temperature.reset();
        self.power_output.reset();
    }

    fn print_debug_info(&self) {
        println!("Battery state:");
        if let Some(temp) = self.temperature.get() {
            println!("  Temperature: {:.2} K", temp.get::<kelvin>());
        } else {
            println!("  Temperature: Not updated");
        }

        if let Some(power) = self.power_output.get() {
            println!("  Power: {:.2} W", power.get::<watt>());
        } else {
            println!("  Power: Not updated");
        }
    }
}

fn main() {
    // Create a battery
    let mut battery = Battery::new();

    // Initialize temperature
    battery
        .temperature
        .update(ThermodynamicTemperature::new::<kelvin>(298.0));

    // Simulation loop
    for step in 0..3 {
        println!("\nTime step {}", step);

        // Reset tracked states at the beginning of each iteration
        battery.reset_tracked_states();

        // Update the battery state
        let power_draw = Power::new::<watt>(1000.0 * (step as f64 + 1.0));
        let time_step = Time::new::<second>(1.0);

        battery.update_state(power_draw, time_step);

        // Print debug info
        battery.print_debug_info();

        // Verify all states were updated
        battery.temperature.assert_updated();
        battery.power_output.assert_updated();
    }
}
