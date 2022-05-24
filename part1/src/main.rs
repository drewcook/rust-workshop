fn main() {
    // Immutability by default
    // let is a keyword for immutable, similar to const in JS
    // use let mut population = 0; to allow it to be reassigned
    let mut city_name = "Rustville";

    // Will always log out ot console.
    // Alternativley, format!("Hello {}", city_name);
    // panic!("I crashed! {}", crash_reason) - this will kill the app
    println!("The city of {}:\n", city_name);
    print_population(1_324_578, 114_293, 108_097);

    city_name = "Cookville";
    println!("\nThe town of {}:\n", city_name);
    print_population(134_578, 4_293, 208_097);
}

fn print_population(adults: u64, kids: u32, buildings: u32)  {
    // 👉 TODO compute population by adding adults to kids
    //
    // 💡 TIP: Use the `as` keyword to convert between numeric types!
    let population: u64 = adults + kids as u64;

    // 👉 TODO compute buildings_per_person by dividing buildings by population
    //
    // 💡 TIP: To get a f64 answer here, both numerator and denominator must be f64 values
    let buildings_per_person = buildings as f64 / population as f64;

    println!("    Population: {}", population * 1);
    println!("        Adults: {}", adults);
    println!("        Kids: {}", kids);
    println!("    Buildings: {}", buildings);
    println!("    Buildings per person: {}", buildings_per_person);

    if buildings_per_person >= 1.0 {
        println!("Everyone can have their own building!");
    } else {
        println!("Buildings must be shared!");
    }
}
