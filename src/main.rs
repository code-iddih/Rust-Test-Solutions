use std::io;

// Function to compute the GCD using Euclidean algorithm
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

// Function to compute the LCM
fn lcm(a: u32, b: u32) -> u32 {
    (a * b) / gcd(a, b)
}

fn main() {
    let mut numbers = Vec::new();
    println!("Enter five numbers:");
    
    for _ in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let num: u32 = input.trim().parse().expect("Please enter a valid number");
        numbers.push(num);
    }
    
    // Computing GCD for all numbers
    let hcf = numbers.iter().copied().reduce(gcd).unwrap();
    // Computing LCM for all numbers
    let lcm_value = numbers.iter().copied().reduce(lcm).unwrap();
    
    println!("HCF (GCD) of the numbers: {}", hcf);
    println!("LCM of the numbers: {}", lcm_value);
}
