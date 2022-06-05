use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Вгадай число!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Будь ласка введіть число.");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Помилка читання рядка");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Помилка вводу!");
                continue;
            },
        };
        
        println!("Ваша спроба: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Занадто мале число"),
            Ordering::Greater => println!("Занадто велике число"),
            Ordering::Equal => {
                println!("Перемога!");
                break;
            }
        }
    }

}
    