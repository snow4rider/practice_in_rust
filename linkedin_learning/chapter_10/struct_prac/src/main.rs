struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }
}

fn main() {
    let mut vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size:7,
        propellant: 0.0
    };

    let vehicle_name = vehicle.get_name();
    println!("vehicle name is {}", vehicle_name);

    println!("propellant is {}", vehicle.propellant);
    vehicle.add_fuel(1000.0);
    println!("propellant is {}", vehicle.propellant);

    let vehicle02 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };

    println!("name is {} the crew size is {} and the propellant is {}",vehicle.name, vehicle.crew_size, vehicle.propellant);
    println!("name is {} the crew size is {} and the propellant is {}",vehicle02.name, vehicle02.crew_size, vehicle02.propellant);
}
