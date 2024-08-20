mod basket;
mod container;

mod stack;

use basket::Basket;
use container::Container;
use stack::Stack;

fn main() {

    let mut b1 = Basket::new("item");
    let b2 = Basket::new(10);
    let b3 = Basket::new(true);

    println!("{:#?}", b1.get());

    let mut s1 = Stack::new(vec![1, 2, 3]);

    println!("{:#?}", s1.get());

}
