enum CitySize {
    Town,       // approximate residents: 1_000
    City,       // approximate residents: 10_000
    Metropolis, // approximate residents: 1_000_000
    Area { residents: u64 } // one with any size
}

struct City {
    description: String,
    residents: u64,
    is_coastal: bool,
}

// A bit more idiomatic style for defining a new city
impl City {
    fn new(city_size: CitySize, is_coastal: bool) -> City {
        // Destructuring from the match return
        let (description, residents) = match city_size {
            CitySize::Town => {
                let residents = 1_000;
                // Returning a tuple, inferred
                (
                    format!("a *town* of approximately {} residents", residents), // i.e. description String
                    residents, // i.e. residents u64
                )
            }
            // 👉 TODO Handle the other CitySize variants individually,
            //    in a similar way to how *town* is handled here
            CitySize::City => {
                let residents = 10_000;
                (
                    format!("a *city* of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::Metropolis => {
                let residents = 1_000_000;
                (
                    format!("a *metro* area of approximately {} residents", residents),
                    residents,
                )
            }
            CitySize::Area { residents } => {
                (
                    format!("an area of people with approximately {} residents", residents),
                    residents
                )
            }
        };
        // Return a new city
        City {
            description,
            residents,
            is_coastal,
        }
    }
}

fn main() {
    // 👉 TODO Use City::new() to create a Metropolis-sized city here
    // Town
    let mut rustville = City::new(CitySize::Town, false);
    println!("This town is {}", rustville.description);
    if rustville.residents > 100_000 {
        println!("Wow!");
    }
    // City
    rustville = City::new(CitySize::City, true);
    println!("This city is {}", rustville.description);
    if rustville.residents > 100_000 {
        println!("Wow!");
    }
    // Metro
    rustville = City::new(CitySize::Metropolis, false);
    println!("This metropolis is {}", rustville.description);
    if rustville.residents > 100_000 {
        println!("Wow!");
    }
    // Area
    rustville = City::new(CitySize::Area { residents: 400 }, true);
    println!("This area is {}", rustville.description);
    if rustville.residents > 100_000 {
        println!("Wow!");
    }
}
