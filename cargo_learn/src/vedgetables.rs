pub mod not_tastey {
    pub struct Cabbage {
        pub price: i32,
    }
    impl Cabbage {
        pub fn print_price(&self) {
            println!("The Cabbage Price is: {:?}", self.price)
        }
    }
}
