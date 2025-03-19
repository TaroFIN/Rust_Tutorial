fn main() {
    let x = 5;
    let y: u32 = 10;
    let z: i32 = -5;
    //By default, all variables are immutable
    //x = 10; // This will give an error

    //To make a variable mutable, use the mut keyword
    let mut a = 5;
    a = 10; // This will work
    println!("The value of a is: {}", a);

    //Another way of assigning variable
    let b = 100;
    println!("The value of b is: {}", b);
    let b = 50;
    println!("The value of b is: {}", b);

    //Multiple variables can be assigned in a single line
    let (name, age) = ("Finn", 28);
    println!("My name is {}, and I am {}", name, age);

    //Constants
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //ID = 002; // This will give an error

    //Shadowing
    let mut c = 5;
    c = c + 1;
    c = c * 2;
    println!("The value of c is: {}", c);
    {
        let c = 10;
        println!("The value of c is: {}", c);
    }
    println!("The value of c is: {}", c);
}   
