// fn main() {
//     enum Option<T> {
//         Some(T),None,
//     }
// }
// fn main() {
//  let a_str = Some(5);
//  let  a_num = Some("Life");
//  let null_num: Option<i32> = None;

// }

// use std::default;

// fn main(){
//         let x = 5;
//         let y: Option<i8> = Some(5);

//         let sum = x+ y.unwrap_or(0);

// }

//USING OPTION ENUMS WITH MATCH EXPRESSSION

// fn main(){
//     let five = Some(5);
//     let six = one_plus(five);
//     let none = one_plus(None); 

// }

// fn one_plus(x: Option<i32>) -> Option<i32> {
//     match  x {
//         // None => None,
//         Some(i)=>  Some(i +1),
//         // IN CASE OF MANY TYPES OF  VALUE USE _
//         _ => None // IF ANY  OTHER PATTER , EXECUTE THIS
//     }
// }

// IF LET SYNTAX

fn main() {

    let some_values = Some(3);
    match some_values{
        Some(3) => println!("Three"),
        _ => (),
    }
    if let Some(3)= some_values {
        println!("three");
    }
}