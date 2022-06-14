struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0}
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let x = vec![1, 2, 3];
    let x1_iter = x.iter();
    
    // for val in x1_iter {
    //     println!("Got: {}", val);
    // }

    // let total: i32 = x1_iter.sum();

    // do nothing, iterators are lazy
    // x1_iter.map(|x| x + 1);


    let x2: Vec<_> =  x1_iter.map(|x| x + 1).collect();

    for elem in Counter::new() {
        println!("Count: {}", elem);
    }
}
