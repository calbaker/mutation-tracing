fn main() {
    println!("Run `cargo test`.  There's nothing happening in here");
}

#[cfg(test)]
mod tests {
    use std::fmt;

    mod tracked_state {
        // A simple TrackedState implementation
        #[derive(Debug, Default)]
        pub struct TrackedState<T: std::fmt::Debug>(Option<T>);

        impl<T> TrackedState<T>
        where
            T: std::fmt::Debug,
        {
            pub fn update(&mut self, value: T) {
                assert!(self.0.is_none());
                self.0 = Some(value);
            }

            pub fn reset(&mut self) {
                self.0 = None;
            }

            pub fn check(&self) {
                assert!(self.0.is_some(), "State variable was not updated!");
            }

            pub fn get(&self) -> Option<&T> {
                self.0.as_ref()
            }
        }
    }

    use tracked_state::TrackedState;

    // Import uom for demonstration
    use uom::si::f64::*;
    use uom::si::power::watt;
    use uom::si::time::second;

    #[test]
    #[should_panic]
    fn test_that_update_can_happen_only_once() {
        let mut pwr = TrackedState::<Power>::default();
        let mut energy = TrackedState::<Energy>::default();
        let mut dt = TrackedState::<Time>::default();

        pwr.update(Power::new::<watt>(1.0));
        dt.update(Time::new::<second>(1.0));
        energy.update(*pwr.get().unwrap() * *dt.get().unwrap());

        pwr.update(Power::new::<watt>(2.0));
    }

    #[test]
    fn test_that_reset_and_check_work() {
        let mut pwr = TrackedState::<Power>::default();
        let mut energy = TrackedState::<Energy>::default();
        let mut dt = TrackedState::<Time>::default();

        pwr.update(Power::new::<watt>(1.0));
        dt.update(Time::new::<second>(1.0));
        energy.update(*pwr.get().unwrap() * *dt.get().unwrap());

        pwr.check();
        dt.check();
        energy.check();

        pwr.reset();
        dt.reset();
        energy.reset();

        pwr.update(Power::new::<watt>(1.0));
        dt.update(Time::new::<second>(1.0));
        energy.update(*pwr.get().unwrap() * *dt.get().unwrap());

        pwr.check();
        dt.check();
        energy.check();
    }
}
