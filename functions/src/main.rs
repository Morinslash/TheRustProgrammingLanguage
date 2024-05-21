use std::i32;

fn main() {
    // print_labeled_measurement(5, 'h');
    let x = five();

    println!("The value of x is: {x}");
} 

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurment is: {x}{unit_label}");
}

fn five() -> i32{
    5
}


