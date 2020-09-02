pub fn get_number() -> i32{
    let number :i32 = loop{
        println!("Please enter a number.");
        let mut input =String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to real line");
        let input: i32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        break input;

    };
    number
}