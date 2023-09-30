pub fn run() {
    println!("Hello from the print.rs file");

    println!("Number: {}", 1);

    println!("{} is from {}", "Samuel", "Aba");

    println!("{0} is from {1} and likes to {2}", "Samuel", "Aba", "Code");

    println!("{name} likes to play {activity}", name="Samuel", activity="Games");

    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    println!("{:?}", (12, true, "Hello"));

    println!("10 + 10 = {}", 10 + 10);
}
