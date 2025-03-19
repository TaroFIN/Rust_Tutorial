fn main() {
    //Arrays these are primitive
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = arr1;
    println!("Values of arr1: {:?}", (arr1,arr2));

    //Vectors these are non-primitive
    let vec1 = vec![1, 2, 3, 4, 5];
    let mut vec2 = &vec1;
    println!("Values of vec1: {:?}", (&vec1,vec2));

}
