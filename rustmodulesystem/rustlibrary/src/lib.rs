// mod front_house {
//     pub mod hosting {
//        pub fn add_to_waitlist (){}
//     }

// }
//     pub fn eat_at_rest(){
//     //Absolute Path
//     crate::front_house::hosting::add_to_waitlist();

//     //Relative Path
//     front_house::hosting::add_to_waitlist();
// }

fn serve_order(){}

mod back_door {
    fn fix_incorrect_order {
        cook_order();
        super:: serve_order();
    }
    fn cook_order(){}
}