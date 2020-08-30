mod functions;

fn main() {
    let tri = functions::Triangle::new(3,4,5);
    println!("{}", tri.is_right())
}
