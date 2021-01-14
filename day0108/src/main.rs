// //fn main() {
// // let x: i32 = 100;
// // let y = x;
// // let x = y;
// // let y = (10, 20);
// // println!("number{}", x);
// // let b = test(999, 999);
// // println!("{}", b);
// // test1(10);
// //}

// // fn test(x: i32, y: i32) -> i32 {
// //     println!("test");
// //     if x + y > 100 {
// //         return x + y;
// //     };
// //     1
// // }

// // fn test1(x: i32) {
// //     let mut i = 0;
// //     while i < x {
// //         println!("{}", i);
// //         i = i + 1;
// //     }
// //     let x = [1, 2, 3, 4, 5];

// //     for temp in x.iter() {
// //         println!("{}", temp)
// //     }
// //     let a = &x[0..3];
// //     for i in a.iter() {
// //         println!("{}", i)
// //     }
// // }

// struct Base {
//     age: u32,
// }
// impl Base {
//     fn test(&self) {
//         println!("{}", self.age)
//     }
//     fn new(age: u32) -> Base {
//         Base { age }
//     }
// }

// enum Hu {
//     x,
//     y,
// }

// impl Hu {
//     fn test()
// }

// fn main() {
//     let abc = Base::new(100);
//     abc.test();
// }
fn main() {
    let r = 5;

    {
        let x = 5;
    }

    println!("r: {}", r);
}
