/*
    Tuple
    let foo: (u32, bool, u8) = (3829, true, 2)
    Struct
    struct Foo { x: i64, is_up: bool };
    Array
    let arr: [u33; 4] = [2, 4, 5, 6];
    Memory
    u8 is 1 byte, u16 is 2 bytes, etc i64 is 64 bits / 8 bytes

    Only arrays are iterable, and must be defined length and type.
    Define order for structs and tuples determines what order they compile down to be stored in memory.
    All three types eventually just end up as bytes in an order in memory.
*/

struct City {
    description: String,
    residents: u64,
    // 👉 TODO add a field here for is_coastal: bool
    is_coastal: bool,
    // 💡 HINT: this will cause other compiler errors.
    //    They will tell you what other changes need to happen!
}

// We're returning an expression, and will be of City type in any case
fn new_city(residents: u64, is_coastal: bool) -> City {
    if is_coastal {
        City {
            description: format!("a *coastal* city of approximately {} residents", residents),
            residents,
            is_coastal,
        }
    } else {
        City {
            residents,
            description: format!("a wonderful inland city with a population of {} residents", residents),
            is_coastal,
        }
    }
}

fn main() {
    let rustville: City = new_city(14222, true);

    println!("This city can be described as: {}", rustville.description);

    if rustville.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }

    let alpinetown: City = new_city(4348, false);

    println!("This city can be described as: {}", alpinetown.description);

    if alpinetown.is_coastal {
        println!("It is a coastal city.");
    } else {
        println!("It is not a coastal city.");
    }
}
