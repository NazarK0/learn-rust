struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: u64,
}

struct Color(i8, i8, i8);

struct AlwaysEqual;


fn main() {
  let user1 = User {
    email: String::from("test@example.com"),
    username: String::from("Jon Doe"),
    active: true,
    sign_in_count: 1,
  };

  let mut user2 = User {
    email: String::from("alucard@example.com"),
    username: String::from("Dracula"),
    active: true,
    sign_in_count: 1,
  };

  user2.email = String::from("changed@gmail.com");

  // println!("user1: {:#?}", user1);

  // struct update syntax
  // we can no longer use user1 after creating user3
  let user3 = User {
    email: String::from("another@example.com"),
    ...user1
  };

  // tuple struct
  let black = Color(0, 0, 0);

  // unit struct
  let subject = AlwaysEqual;

}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

