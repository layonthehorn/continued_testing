use std::collections::HashMap;

pub struct Dictionary{
    hash_map: HashMap<String, i32>,

}

impl Dictionary{
    pub fn new(&self) -> Dictionary{
        Dictionary{
            hash_map: Default::default()
        }
    }

    pub fn add_key(&mut self, key: &str, number: i32){
        if !self.hash_map.contains_key(key){
            self.hash_map.insert(key.to_string(), number);
        }
        else{
            panic!("Key already exists")
        }
    }

    pub fn delete_key(&mut self, key: &str){
        if !self.hash_map.contains_key(key){
            self.hash_map.remove(key);
        }
        else{
            panic!("Key Does not exist")
        }
    }

    // gets a value from the dictionary or returns an error is unsuccessful.
    pub fn get_value(&self, value: &str) -> Option<&i32> {
            return self.hash_map.get(value)
    }

}