// Vectors

//Recall you cannot add items to array. You can only change the value of the items in the array.
//Vectors are similar to arrays but you can add items to them.

fn main() {
    // Vectors are resizable arrays
    // Vectors can only store items of the same type
    let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Create a new empty vector
    let mut v2: Vec<i32> = Vec::new();

    // Operation
    v[1] = 20;
    println!("v[1] = {}", v[1]);

    // Add an item to the vector
    v2.push(6);
    println!("v2 = {:?}", v2);

    // Get the length of the vector
    println!("Length of v = {}", v.len());

    // Pop an item from the vector
    let popped = v.pop();
    println!("Popped item = {:?}", popped);

    // Iterate over the vector
    for i in &v{
        println!("{}", i);
    }

    // Iterate over the vector, but we can change the value of the items
    for i in v.iter_mut(){
        *i += 10;
        println!("{}", i);
    }

    println!("v = {:?}", v);
}
