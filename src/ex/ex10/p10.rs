use std::{
    fmt::{Debug, Display},
    iter::Sum,
};

pub fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item
        }
    }
    largest
}

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}-{}", item1.summarize(), item2.summarize())
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> Pair<T> where T: Display + PartialOrd {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x={}", self.x);
        } {
            println!("The largest number is y={}", self.y);
        }
    }
}

pub fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("i={}", i);
            return &s[0..i]
        }
    }
    println!("----------------");
    return &s[..]
}

pub struct Guess {
    value: i32
}

impl Guess {
    pub fn new(value: i32) -> Self {
        if value < 1 || value > 100 {
            panic!(
                "Guess value must be between 1 and 100, got {}",
                value
            )
        }
        Guess {value}
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(200);
    }
}