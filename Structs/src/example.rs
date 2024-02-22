fn main(){
    let rect = (30 , 30);
    
    println!("{}",area(rect));
}
fn area(dimensions:(u32, u32)) -> u32 {

dimensions.0 * dimensions.1
}