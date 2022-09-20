// use std::io::{self, Write};
// use std::ops;
//
// struct Foo {
//     x: i32,
// }
// struct Bar;
//
// #[derive(Debug)]
// struct FooBar;
//
// struct MyTest;
//
// #[derive(Debug)]
// struct BarFoo;
//
// impl ops::Add<Foo> for Foo {
//     type Output = Foo;
//
//     fn add(self, _rhs: Foo) -> Foo {
//         println!("> Foo.add(Bar) was called");
//         println!("> Current value of self.x: {}", self.x + _rhs.x);
//         // FooBar
//         // MyTest
//         Foo{x: self.x + _rhs.x}
//     }
// }
//
// fn main() {
//     // Quick operator overload testing
//     // println!("{:?}", Foo + Bar);
//
//     let test1 = Foo { x: 42 } + Foo { x: 42 };
//     println!("Now that we added the same struct the value is: {:?}", test1.x);
// }