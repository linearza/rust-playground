fn main() {
  let x = 5;
  let x = x + 2;
  {
    let x = 6;
    println!("The value of x in inner scope is {}", x);

    {
      let x = 12;
      println!("The value of x in inner inner scope is {}", x);
    }
  }

  println!("The value of x is {}", x);

  // example of shadowing and changing type
  let spaces = "      "; // string
  let spaces = spaces.len(); // number

  println!("There are {} spaces", spaces);
}
