// generic function to get both floats and integers
pub fn get_number<T>(message: &str) -> T
// restricts the function to only work with types that implement
// the from str function.
where
    T: std::str::FromStr,
{
    let number: T = loop {
        println!("{}", message);
        let mut input = String::new();
        // attempts to read the line to get input
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to real line");
        // checks the return of parse to see if it worked or not
        let input: T = match input.trim().parse() {
            // if so it returns the number inside
            Ok(num) => num,
            // otherwise it just asks again
            Err(_) => continue,
        };
        // breaks the loop and returns the value
        break input;
    };
    // returns the final number given
    number
}
