fn main() {
    let mut pa = Person::new("Jun", 29, "Shinsen");

    pa.print();
    pa.print_t(true);
    pa.print_t(false);
    println!("{}", pa.to_str());
    pa.add_age(1);
    println!("{}", pa.to_str());

    let maaya = Person::new("maaya", 27, "Sancha");
    println!("{}", maaya.to_str());
}

struct Person {
    id: i32,
    name: String,
    age: i32,
    addr: String,
}
static mut PERSON_ID: i32 = 0;

impl Person {
    fn new(name: &str, age: i32, addr: &str) -> Person {
        let new_id = unsafe {
            PERSON_ID += 1;
            PERSON_ID
        };

        Person {
            id: new_id,
            name: name.to_string(),
            age: age,
            addr: addr.to_string(),
        }
    }
    fn print(&self) {
        println!("{}, {}, {}, {}", self.id, self.name, self.age, self.addr);
    }

    fn print_t(&self, private: bool) {
        if private {
            println!("{}, {}", self.id, self.name);
        } else {
            println!("{}, {}, {}, {}", self.id, self.name, self.age, self.addr);
        }
    }

    fn to_str(&self) -> String {
        format!("{}, {}({}), {}", self.id, self.name, self.age, self.addr)
    }

    fn add_age(&mut self, n: i32) {
        self.age += n;
    }
}
