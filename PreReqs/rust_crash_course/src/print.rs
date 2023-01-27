pub fn run() {
    // Print to console
    println!("Printing Hello World from print.rs");

    // Basic formatting
    println!("{} is the a {}", "Jack", "boy");

    // Positional Arguments
    println!("{0} is a good {1} and {1} are {2}", "Jack", "boy", "great");

    // Named Arguments
    println!("{first} is a great {second}", first="Jack", second="person");

    // Placeholder traits
    println!("Hex = {:x}, Binary = {:b}, Octa = {:o}", 10, 10, 10);

    // Debugger Traits
     println!("{:?}", (true, "Person", 10));

    // Simple Math
    println!("10 + 10 = {}", 10 + 10)

}