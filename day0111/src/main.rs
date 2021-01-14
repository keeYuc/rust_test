fn main() {
    // println!("Hello, world!");
    // test(3);
    // test("sdad");
    let x = Human::new(String::from("saddasdsada"));
    x.print_name();
    test_life(&String::from("adadad"), &String::from("Sadada"));
}

// fn test<T>(abc: T) {
//     println!("sss");
//     let a = Person::new("sadadad", 100);
//     let b = Person::new(224224, 1_9);
//     print_id(&a);
//     print_id(&b);
// }

// struct Person<T> {
//     unuse: T,
//     id: u32,
// }

// impl<T> Person<T> {
//     fn new(abc: T, id: u32) -> Person<T> {
//         Person { unuse: abc, id }
//     }
// }
// impl<T> GetId for Person<T> {
//     fn get_id(&self) -> u32 {
//         self.id
//     }
// }

// trait GetId {
//     fn get_id(&self) -> u32;
// }

// trait GetIdPlus {}

// fn print_id<T: GetId>(s: &T) {
//     println!("{}", s.get_id());
// }

// fn print_id(s: &impl GetId) {
//     println!("{}", s.get_id());
// }

struct Human {
    name: String,
}

trait GetName {
    fn get_name(&self) -> &str;
}

trait PrintName {
    fn print_name(&self);
}

impl<T: GetName> PrintName for T {
    fn print_name(&self) {
        println!("{}", self.get_name());
    }
}
impl GetName for Human {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl Human {
    fn new(name: String) -> Human {
        Human { name }
    }
}

fn test_life<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a > b {
        return a;
    } else {
        return b;
    }
}
