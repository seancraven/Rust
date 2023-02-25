use crate::vedgetables::not_tastey::Cabbage;

pub mod vedgetables;

fn main() {
    let special_cbbg = Cabbage { price: 40 };

    special_cbbg.print_price()
}
