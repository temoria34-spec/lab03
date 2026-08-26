use std::io;

fn main() {
    let mut input = String::new();

    // Output "Enter distance"
    println!("What is the distance of your trip in miles?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let distance: i32 = input.trim().parse().expect("Not a number");

    input = String::new();

    // Output "Enter distance"
    println!("What is the miles per gallon of your car or tuck?");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("input: {input}");
    let mpg: f64 = input.trim().parse().expect("MPG - Not a number");

    input = String::new();

    // Output "Enter distance"
    println!("What is the price per gallon of gas currently? (In dollars)");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let ppg: f64 = input.trim().parse().expect("Not a number");
    // Type conversion for distance to integer
    let gallons = distance as f64 / mpg;
    let total_cost = gallons * ppg;
    println!("Gallons needed: {}", gallons);
  println!("Total Cost (In Dollars): {}", total_cost);
  println!("Distance: {}", distance);
  println!("Efficiency (mpg): {}", mpg);
println!("Price Per Gallon: {}", ppg);
}


