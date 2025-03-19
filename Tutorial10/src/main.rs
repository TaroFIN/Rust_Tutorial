//Enum
// Structs and enums are the building blocks for creating new types in Rust.

// Enums allow us to enumerate a list of variants

mod file1;

enum IpAddressKind {
    V4(u8, u8, u8, u8),
    V6,
}

enum Message{
    Quit,
    Move{x:i32, y:i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T>{
    Some(T),
    None,
}

enum Movement{
    Jump,
    Left,
    Right,
}

struct IpAddress{
    kind: IpAddressKind,
    address: String,
}

struct QuitMessage; //unit struct
struct MoveMessage{
    x: i32,
    y: i32,
}
struct WriteMessage(String); //tuple struct
struct ChangeColorMessage(i32, i32, i32); //tuple struct

impl Message{
    fn function1(){
        println!("Code of the future!");
    }
}

fn move_player(m: Movement){
    match m {
        Movement::Jump => println!("Jumping"),
        Movement::Left => println!("Left"),
        Movement::Right => println!("Right"),
    }
}

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    fn route(ip_kind: IpAddressKind){

    }  

    // let localhost = IpAddress{
    //     kind: IpAddressKind::V4,
    //     address: String::from("127.0.0.1"),
    // }; 

    let localhost = IpAddressKind::V4(127, 0, 0, 1);

    //Option Enum
    let number = Some(5);
    let boolean = Some(true);
    let nothing: std::option::Option<i32> = None;
    let something: std::option::Option<i32> = Some(5);

    // They are not same type, so we can't add them
    // let x: i32 = 14;
    // let y = x + something;
    // println!("{}", y);

    let x: i32 = 14;
    let y = x + something.unwrap_or(0);
    println!("{}", y);

    let player1 = Movement::Jump;
    let player2 = Movement::Left;
    let player3 = Movement::Right;
    move_player(player1);
    move_player(player2);
    move_player(player3);

    file1::maths();
}
