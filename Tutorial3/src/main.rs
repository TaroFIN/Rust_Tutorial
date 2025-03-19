fn main() {
    let x: i32 = 5;

    println!("The value of x is {}", x);
    println!("The maximum value of u32 is {}", std::u32::MAX);
    println!("The mininum value of i32 is {}", std::i32::MIN);

    let y: f32 = 3.5;
    println!("The value of y is {}", y);
    println!("The maximum value of f32 is {}", std::f32::MAX);
    println!("The mininum value of f32 is {}", std::f32::MIN);

    // Boolean
    let mut true_false: bool = true;
    println!("The value of true_false is {}", true_false);
    true_false = false;
    println!("The value of true_false is {}", true_false);

    //Character
    let mut character: char = 'a';
    println!("The value of character is {}", character);
    character = 'b';
    println!("The value of character is {}", character);
    character = '\u{1F600}';
    println!("The value of character is {}", character);

    // Tuple
    let mut tuple1: (i32, f32, char) = (5, 3.5, 'a');
    println!("The value of tuple1 is {:?}", tuple1);
    tuple1.0 = 10; tuple1.1 = 4.5; tuple1.2 = 'b';
    println!("The value of tuple1 is {:?}", tuple1);

    // Array
    let mut array1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of array1 is {:?}", array1);
    array1[0] = 10; array1[1] = 20; array1[2] = 30; array1[3] = 40; array1[4] = 50;
    println!("The value of array1 is {:?}", array1);

    // Slice
    let slice1: &[i32] = &array1[1..4];
    println!("The value of slice1 is {:?}", slice1);
} 