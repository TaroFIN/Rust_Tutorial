// Main function, generally the entry point of the program
fn main() {
    first_function();
    second_function(5, 3.5);
    let res: i32 = third_function(5, 3);
    println!("The result of third_function is {}", res);

    // variable declaration
    let z: i32 = {
        let x: i32 = 5;
        x + 1
    };
    println!("The value of z is {}", z);

    //Closure
    let closure = |x: i32, y: i32| -> i32 {
        x + y
    };
    let res_closure = closure(5, 3);
    println!("The result of closure is {}", res_closure);
}

// Create own function
// Function without parameters
// The function signature is fn function_name()
// The function body is enclosed by curly braces
fn first_function() {
    println!("This is the first function");
}

// Function with parameters
// The parameters are specified in the function signature
// The type of the parameters are specified after the parameter name
// The type of the parameters are separated by a comma
// The type of the parameters are specified after the colon symbol
// The type of the parameters are i32 and f32
fn second_function(x: i32, y: f32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}

// Function with return value
// The return type is specified after the -> symbol
// The return type is i32
fn third_function(x: i32, y: i32) -> i32 {
    x + y
}