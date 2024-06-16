use p10::p10::first_word;

#[path ="../ex/ex10/mod.rs"]
mod p10;

fn main() {
    let num_list = vec![34, 50, 25, 100, 65];
    println!("{}", p10::p10::largest(&num_list));
    println!("first word:{}", first_word("This is a story"));
}