// Control flow in Rust

fn main() {
    // if statement
    let number = 5;
    if number < 5 {
        println!("condition was true");
    } 
    else if number == 5
    {
        println!("condition was equal to 5");
    }
    else 
    {
        println!("condition was false");
    }

    let condi = 5>=7;
    println!("{}", condi);

    // Types must be the same, otherwise it will throw an error
     let condi2 = (5 as f32)>=3.2;
     println!("{}", condi2);

    // if statement in let
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // loop
    let mut counter = 0;
    loop{
        counter+=1;
        if counter == 10{
            break;
        }
    };
    println!("The result is: {}", counter);

    // while loop
    let mut count = 0;
    while count<=10{
        if count%15==0{
            println!("FizzBuzz");
        }
        else if count%3==0 {
            println!("Fizz");
        }
        else if count%5==0 {
            println!("Buzz");
        }
        else{
            println!("{}", count);
        }
        count +=1;
    }

    // for loop
    for x in 0..10 {
        println!("{}", x);
    }

    // Iterating loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter(){
        println!("The value is: {}", element);
    }
}
