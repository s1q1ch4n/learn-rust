use std::io;

fn main() {
    println!("Enter your weight(kg):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    // println!("{}", input);

    let weight: f32 = input.trim().parse().unwrap();
    // println!("{}", weight);

    let mut mars_weight = calculate_weight_on_mars(weight);
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}kg", mars_weight);

    // borrow_string(&input);
    // own_string(input);
}

fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

fn borrow_string(s: &String) {
    println!("{}", s);
}

fn own_string(s: String) {
    println!("{}", s);
}
