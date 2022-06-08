enum IpAddrKind {
  v4,
  v6,
}

enum IpAddress {
  v4(u8, u8, u8, u8),
  v6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32},
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    // method body
  }
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
  Iowa,
  Texas
}

struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

fn main() {
  let four = IpAddrKind::v4;
  let six = IpAddrKind::v6;

  let home = IpAddr {
    kind: IpAddrKind::v4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::v6,
    address: String::from(":::1")
  };

  let home2 = IpAddress::v4(127, 0, 0, 1);
  let loopback2 = IpAddress::v6(String::from(":::1"));

  let m = Message::Write(String::from("Hello"));
  m.call();

  // the Option Enum
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_mumber: Option<i32> = None;

  // let x: i8 = 6;
  // let y: Option<i8> = Some(6);
  // let sum = x + y;

  let two = Some(2);
  let three = plus_one(two);
  let none = plus_one(None);

  let config_max = Some(3u8);

  // using match
  match config_max {
    Some(max) => println!("The maximum is cofigured to be {}", max),
    _ => (),
  }

  // using if let
  if let Some(max) = config_max {
    println!("The maximum is configured to be {}", max);
  }

}

fn route(ip_kind: IpAddrKind) {

}

fn value_in_cents(coin: Coin) -> u8 {
  match coin {
    Coin::Penny => {
      println!("Lucky penny!");
      1
    },
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}
