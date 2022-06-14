use std::{thread, time::Duration, collections::HashMap, hash, fmt};

struct Cacher<A,R,F>
where
    F: Fn(A) -> R,
    A: Eq + PartialEq + hash::Hash + Copy,
    R: Copy,
{
    calculation: F,
    cache: HashMap<A, R>,
}

impl<A,R,F> Cacher<A,R,F>
where
    F: Fn(A) -> R,
    A: Eq + PartialEq + hash::Hash + Copy + fmt::Display,
    R: Copy+fmt::Display,
{
    fn new(calculation: F) -> Cacher<A,R,F> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: A) -> R {
        match self.cache.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.cache.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
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
            expensive_result.value(intensity-2)
        );

        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity-5)
        );

        println!(
            "Next, do {} pullups!",
            expensive_result.value(intensity-5)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            let run_intensity = ((intensity as f64) * 1.12).round() as u32;
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(run_intensity)
            );
        }
    }
}
