mod functions;

fn main() {
    let mut tri = functions::Triangle::new(3,4,5);
    println!("Sides are: {}, {}, and {}.", tri.get_side_one(), tri.get_side_two(), tri.get_side_three());
    println!("Is right: {}", tri.is_right());
    println!("Is equilateral: {}", tri.is_equilateral());
    println!("Is scalene: {}", tri.is_scalene());
    println!("Is isosceles: {}", tri.is_isosceles());
    tri.set_side_one(5);
    tri.set_side_two(5);
    tri.set_side_three(5);
    println!("Sides are: {}, {}, and {}.", tri.get_side_one(), tri.get_side_two(), tri.get_side_three());
}
