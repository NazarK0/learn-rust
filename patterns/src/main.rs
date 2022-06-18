struct Point {
   x: i32,
   y: i32,
}

struct Point3D {
   x: i32,
   y: i32,
   z: i32,
}

enum Color {
   Rgb(i32, i32, i32),
   Hsv(i32, i32, i32),
}

enum Message {
   Quit,
   Move { x: i32, y: i32 },
   Write(String),
   ChangeColor(Color),
}

enum MessageAt {
   Hello { id: i32 },
}

// Ignoring an Entire Value with _
fn foo(_: i32, y:i32) {
   println!("This code only uses the y parameter: {}", y);
}

fn main() {
   let a = Some(5);
   let b = 10;


   match a {
      Some(50) => println!("Got 50"),
      Some(y) => println!("Matched, b={:?}", b),
      _ => println!("Default case, a = {:?}", a),
   }

   println!("at the end: a={:?}, b={:?}", a, b);
   
   // Multiple Patterns
   let z = 1;

   match z {
      1 | 2 => println!("Got 1 or 2"),
      3 => println!("Got 3"),
      _ => println!("anything"),
   }

   // Matching Ranges of Values
   let u = 5;

   match u {
      // u is equal 1 or 2 or 3 or 4 or 5
      // allowed for numeric or char values
      1..=5 => println!("one through five"),
      _ => println!("something else"),
   }

   let c = 'c';

   match c {
      'a'..='j' => println!("early ASCII letter"),
      'k'..='z' => println!("late ASCII letter"),
      _ => println!("something else"),

   }

   // Destructuring to Break Apart Values
   let p = Point { x: 0, y: 7 };
   let Point { x: d, y: e } = p;
   let Point { x, y } = p;
   assert_eq!(0, d);
   assert_eq!(0, x);
   assert_eq!(7, e);
   assert_eq!(7, y);

   match p {
      Point { x, y: 0 } => println!("On the x axis at {}", x),
      Point { x: 0, y } => println!("On the y axis at {}", y),
      Point { x, y } => println!("On neither axis: ({}, {})", x, y),
   }

   // enums
   let msg =  Message::ChangeColor(Color::Hsv(0, 150, 223));

   match msg {
      Message::Quit => {
         println!("quit")
      }
      Message::Move { x, y } => {
         println!("Move to ({}, {})", x, y);
      }
      Message::Write(text) => println!("Text message: {}", text),
      // Destructuring Nested Structs and Enums
      Message::ChangeColor(Color::Rgb(r, g, b)) => println!("Change color to rgb({}, {}, {})", r, g, b),
      Message::ChangeColor(Color::Hsv(h, s, v)) => println!("Change color to hsv({}, {}, {})", h, s, v),
   }

   // Destructuring Structs and Tuples
   let ((feet, inches), Point { x, y } ) = ((3, 10), Point { x: 1, y: -10 });
   println!("destuct feet= {}", feet);
   println!("destuct inches= {}", inches);
   println!("destuct x= {}", x);
   println!("destuct y= {}", y);

   // Ignoring Values in a Pattern
   foo(3, 4);

   // Ignoring Parts of a Value with a Nested _
   let mut setting_value = Some(5);
   let new_setting_value = Some(10);

   match (setting_value, new_setting_value) {
      (Some(_), Some(_)) => {
         println!("Can`t overwrite an exitsting customized value");
      }
      _ => {
         setting_value = new_setting_value;
      }
   }
   
   println!("setting is {:?}", setting_value);

   let numbers = (2, 4, 8, 16, 32);

   match numbers {
      (first, _, third, _, fifth) => {
         println!("Some numbers: {}, {}, {}", first, third, fifth)
      }
   }
   
   // Ignoring an Unused Variable by Starting Its Name with _
   let f = 11;
   let _g = 22;

   // Ignoring Remaining Parts of a Value with ..
   let origin = Point3D{ x: 2, y: 44, z: 11};

   match origin {
      Point3D {x, ..} => println!("x is {}", x),
   }

   let numbers = (2, 4, 8, 16, 32);

   match numbers {
      (first, .., last) => {
         println!("Some numbers: {}, {}", first, last);
      }
   }

   // Extra Conditionals with Match Guards
   let num = Some(4);

   match num {
      Some(x) if x % 2 == 0 => println!("The number {} is even", x),
      Some(x) => println!("The number {} is odd", x),
      None => (),
   }

   let h = 4;
   let m = false;

   match h {
      // if m match guard applies to 4 , 5 , and 6
      4 | 5 | 6 if m => println!("yes"),
      _ => println!("no"),
   }

   // @ Bindings
   let msg = MessageAt::Hello { id: 5 };

   match msg {
      MessageAt::Hello {
         id: id_variable @ 3..=7, 
      } => println!("Found an id in range: {}", id_variable),
      MessageAt::Hello {
         id: 10..=12
      } => println!("Found id in another range"),
      MessageAt::Hello { id } => println!("Some other id: {}", id),
   }

}
