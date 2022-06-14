use std::{thread, time::Duration, collections::HashMap};

#[derive(Debug, Hash, Eq)]
struct Cacher<A,T,F>
where
    F: Fn(A) -> T,
{
    calculation: F,
    cache: HashMap<A, T>,
    value: Option<T>,
}

impl<A,T,F> Cacher<A,T,F>
where
    F: Fn(A) -> T,
{
    fn new(calculation: F) -> Cacher<A,T,F> {
        Cacher {
            calculation,
            cache: HashMap::new(),
            value: None,
        }
    }

    fn value(&mut self, arg: A) -> T {

        self.cache.entry(arg).or_insert((self.calculation)(arg));
        // match self.cache.get(arg) {
        //     Some(v) => v,
        //     None => {
        //         let v = (self.calculation)(arg);
        //         self.value = Some(v);
        //         v
        //     }
        // }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(3));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num: u32| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(3));
        num
    });

    if intensity < 25 {
        println!(
            "Today do {} pushups!",
            expensive_result.value(intensity)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
