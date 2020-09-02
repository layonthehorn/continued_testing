pub trait GetInfo {
    fn get_full_name(&self) -> String;
    fn get_age(&self) -> i32;
}

pub struct Person {
    name: [String; 2],
    age: i32,
}

impl Person {
    pub fn new(first_name: &str, last_name: &str, years: i32) -> Person {
        Person {
            name: [first_name.to_string(), last_name.to_string()],
            age: years,
        }
    }
}

impl GetInfo for Person {
    fn get_full_name(&self) -> String {
        format!("{} {}", self.name[0], self.name[1])
    }
    fn get_age(&self) -> i32 {
        self.age
    }
}

pub struct Dog {
    name: [String; 2],
    age: i32,
}

impl Dog {
    pub fn new(first_name: &str, last_name: &str, years: i32) -> Dog {
        Dog {
            name: [first_name.to_string(), last_name.to_string()],
            age: years,
        }
    }
}

impl GetInfo for Dog {
    fn get_full_name(&self) -> String {
        format!("{} {}", self.name[0], self.name[1])
    }
    fn get_age(&self) -> i32 {
        self.age
    }
}

pub struct Cat {
    name: [String; 2],
    age: i32,
}

impl Cat {
    pub fn new(first_name: &str, last_name: &str, years: i32) -> Cat {
        Cat {
            name: [first_name.to_string(), last_name.to_string()],
            age: years,
        }
    }
}
impl GetInfo for Cat {
    fn get_full_name(&self) -> String {
        format!("{} {}", self.name[0], self.name[1])
    }
    fn get_age(&self) -> i32 {
        self.age
    }
}
