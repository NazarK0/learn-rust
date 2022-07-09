
fn main() {
    let mut counter = 55;

    let sqrt = 'outer: loop {
        let n = next_number(&mut counter);
        println!("number: {}", n);

        for i in 1.. {
            let square = i * i;
            println!("square: {square}");

            if square == n {
                print!("\n\n");
                break 'outer i;
            }

            if square > n {
                print!("\n\n");
                break;
            }
        }
    };


    println!("sqrt:{}, number:{}", sqrt, counter);
}

fn next_number(number: &mut u32) -> u32 {
    *number += 1;

    *number
}
