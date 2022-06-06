fn main() {
    let numbers = vec![2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24];
    let numbers_ref: &Vec<i64> = &numbers;
    let numbers_slice: &[i64] = numbers.as_slice(); // could've also done &numbers[0..100]
    let sum_of_nums = sum(numbers_ref);
    let product_of_nums = product(numbers_slice); // 👉 TODO call product() and put answer here
    // Rust supports passing in a ref to a function that takes in a slice, as we do here
    let average_of_nums = average(&numbers); // 👉 TODO call average() and put answer here

    // 💡 TIP: You'll get a compile error. Here are two ways you can fix it:
    //
    // Option 1: Accept &Vec<i64> instead of Vec<i64>
    //
    // Option 2: Accept a slice - that is, &[i64] - instead of a Vec'
    // - This is the better option because if the function takes in a slice, it can support both passing a slice in but also a vec.
    //   If the function were defined with a reference parameter to a vec, it could only support a vec being passed in, but not a slice.
    // Give both options a try!

    println!("Sum of these numbers: {}", sum_of_nums);
    println!("Product of these numbers: {}", product_of_nums);
    println!("Average of these numbers: {}", average_of_nums);
}

// Using option 1 with a reference just for demonstration
fn sum(numbers: &Vec<i64>) -> i64 {
    let mut total = 0;

    for num in numbers.iter() {
        total += num;
    }

    total
}

// Use option 2 with a slice
fn product(numbers: &[i64]) -> i64 {
    let mut total = 1;

    for num in numbers.iter() {
        total *= num;
    }

    total
}

// Use option 2 with a slice
fn average(numbers: &[i64]) -> i64 {
    let length = numbers.len() as i64;
    // Convert back to a vec reference for our sum method
    // (this normally would also just use a slice as well, but I wanted to demonstrate using a ref param)
    let numbers_vec: &Vec<i64> = &numbers.to_vec();
    sum(numbers_vec) / length
}
