use std::slice;

unsafe fn dangerous() {}

// Accessing or Modifying a Mutable Static Variable
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// example how might be implemented this function in std namespace
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

// Calling Rust Functions from Other Languages
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Implementing an Unsafe Trait
unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // create a raw pointer to an arbitrary location in memory
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        dangerous();
    }

    // Creating a Safe Abstraction over Unsafe Code
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Using extern Functions to Call External Code
    unsafe {
        println!("absolute value of -3 according to C: {}", abs(-3));
    }

    // Accessing or Modifying a Mutable Static Variable
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

}
