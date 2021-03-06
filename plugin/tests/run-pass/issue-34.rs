#![feature(plugin)]
#![plugin(mutagen_plugin)]
#![feature(custom_attribute)]

extern crate mutagen;

fn foo() -> usize { 2 }
fn bar() -> usize { 23 }

#[mutate]
fn blubb() -> usize {
    let mut value;
    if ({ value = foo(); value > 5 }) || ({ value = bar(); value < 5 }) {
        value + 1
    } else {
        42
    }
}

fn main() {
    println!("{}", blubb());
}