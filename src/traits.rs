pub trait GetFullName{
    fn get_full_name(&self) -> String;
}


pub struct Person{
    name: [String; 2],
    age: i32
}

impl Person{
    pub fn new(first_name: String, last_name: String, years: i32) -> Person{
       Person{
           name: [first_name, last_name],
           age: years
       }

    }
}

impl GetFullName for Person{
    fn get_full_name(&self) -> String{
        format!("{} {}", self.name[0], self.name[1])
    }
}

pub struct Dog{
    name: [String; 2],
    age: i32
}

impl Dog{
    pub fn new(first_name: String, last_name: String, years: i32) -> Dog{
        Dog{
            name: [first_name, last_name],
            age: years
        }

    }
}

impl GetFullName for Dog{
    fn get_full_name(&self) -> String{
        format!("{} {}", self.name[0], self.name[1])
    }
}
pub struct Cat{
    name: [String; 2],
    age: i32
}

impl Cat{
    pub fn new(first_name: String, last_name: String, years: i32) -> Cat{
        Cat{
            name: [first_name, last_name],
            age: years
        }

    }
}
impl GetFullName for Cat{
    fn get_full_name(&self) -> String{
        format!("{} {}", self.name[0], self.name[1])
    }
}
