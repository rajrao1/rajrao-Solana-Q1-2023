pub fn run() {

    // Variables

    let name = "James";
    let age;

    age = 39;

    println!("{} is aged {}", name, age);


    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (the_name, the_age) = ("Alex", 40);
    println!("{}  and age is {}", the_name, the_age);
}