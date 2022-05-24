struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64
}

fn main() {
    let vehicle = Shuttle{
        name: String::from("Endeavour"),
        crew_size:7,
        propellant: 835958.0
    };

    println!("name is {} the crew size is {} and the propellant is {}",vehicle.name, vehicle.crew_size, vehicle.propellant);
}
