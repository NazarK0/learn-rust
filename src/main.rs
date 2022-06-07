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
    kind: IpAddrKind:v6,
    addess: String::from(":::1")
  }

  let home2 = IpAddress::v4(127, 0, 0, 1);
  let loopback2 = IpAddress::v6(String::from(":::1"));

  let m = Message::Write(String::from("Hello"));
  m.call();

  // the Option Enum
  let some_number = Some(5);
  let some_string = Some("a string");

  let absent_mumber: Option<i32> = None;

}

fn route(ip_kind: IpAddrKind) {

}
