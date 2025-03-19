fn main() {
    println!("Hello, world! My name is {} and I love {}", "Finn", "Rust");
    println!("{}", 4);  //You can't print(4) instead, it would not work.
    println!("This is placeholder line. This is {0}, {0} is violated {1} everyone {2} it.", "weed", "by", "loves");
    println!("This is incorporating line. My name is {name} and I am {age}", name="Finn", age="28");
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    println!("Calculation {}+{}={}", 1, 2, 1+2);

    //Tiny rustfmt test
    let x = 5;
    if x == 5 {
        println!("x is five!");
    }
    else {
        println!("x is not five!");
    }

    let y: u32 = 42;
    
}
