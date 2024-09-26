// fn main() {
//     println!("{}", is_even(22));
// }

// fn is_even(num: i32)-> bool {
//     if num % 2 == 0 {
//         return  true;
//     }
//     else {
//         return  false;
//     }
// }

// FIBONAKI SERIES

// fn main() {
//     print!( "{}", fib(12));
// }

// fn fib (num: u32) -> u32 {
//     let mut first = 0;
//     let mut second = 1;


//     if num == 0 {
//         return first;
//     }
    
//     if num == 1 {
//         return  second;
//     }

//     for _ in  0..(num - 1) {
//         let temp = second;
//         second = second + first;
//         first  = temp;
//     }
//     return  second;

// }


fn main() {
    let name = String::from("aniket");
    let len = get_str_len(str:name);
    print!("The len of this str is {}", len);
}

fn get_str_len(str: String) -> usize {
    return  str.chars().count();
}