fn main() {
  println!(r"cargo:rustc-link-search=native=/home/nazar/projects/rust/learn-rust/libgit2-1.5.0/build");
}

// on run:
// $ export LD_LIBRARY_PATH=/home/jimb/libgit2-0.25.1/build:$LD_LIBRARY_PATH
// $ cargo run