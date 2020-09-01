mod triangle;
mod traits;
use crate::traits::GetFullName;

fn main() {
    let test_type: String = get_test_type();
    if test_type == "1"{
        let mut tri = triangle::Triangle::new(3, 4, 5);
        println!("Sides are: {}, {}, and {}.", tri.get_side_one(), tri.get_side_two(), tri.get_side_three());
        println!("Is right: {}", tri.is_right());
        println!("Is equilateral: {}", tri.is_equilateral());
        println!("Is scalene: {}", tri.is_scalene());
        println!("Is isosceles: {}", tri.is_isosceles());
        tri.set_side_one(5);
        tri.set_side_two(5);
        tri.set_side_three(5);
        println!("Sides are: {}, {}, and {}.", tri.get_side_one(), tri.get_side_two(), tri.get_side_three());
    } else if test_type == "2"{

        // creates three new objects with the trait GetFullName
        let man = traits::Person::new("Darren".to_string(), "Capper".to_string(), 27);
        let canine = traits::Dog::new("Berk".to_string(), "Bomber".to_string(), 4);
        let feline = traits::Cat::new("Vern".to_string(), "MacCaster".to_string(), 27);

        // creates a vector of them and specifies that they share that trait,
        // dynamically.
        let group: Vec<&dyn GetFullName> = vec! [&man, &canine, &feline];
        // iterates over them and prints their names
        for object in group.iter(){
            println!("{}", object.get_full_name());
        }

    }
}

fn get_test_type() -> String{
    // gets the second argument
    let args  = std::env::args().nth(1).unwrap_or_else(|| {
        // if there is not a second one it defaults to 1
        let val = "1".to_string();
        val
    });
    args


}