pub fn run(){
    use std::io;
    use rand::Rng;
    use std::cmp::Ordering;

    let secret_number = rand::thread_rng().gen_range(1,101);
    // println!("The secret number is: {}", secret_number);
    let mut guess = String::new();
    loop {
        println!("Enter a guess:");
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {println!("you won"); break;},
        }
        println!("secret is {}",secret_number);
    }
}