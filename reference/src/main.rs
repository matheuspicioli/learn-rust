fn main() {
    let s1 = String::from("Hello");
    let mut s2 = String::from("Hello s2");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // it doesn`t work!
    change_imut_ref(&s1);

    // it works!
    change_mut_ref(&mut s2);

    let mut s3 = String::from("Hello, world!");

    // it doesn`t work! more than one mutable references at same time
    let r1 = &mut s3;
    let r2 = &mut s3;

    // println!("{}, {}", r1, r2);

    // it works!
    {
      let r1 = &mut s3;
      println!("r1: {}", r1);
    } // r1 goes out of scope here! it's not multiple reference at same time

    let r2 = &mut s3;
    println!("r2: {}", r2);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn change_imut_ref(s: &String) {
  // it doesn`t work!
  // s.push_str("something");
  println!("New string: {}", s);
}

fn change_mut_ref(s: &mut String) {
  // it works!
  s.push_str(" world!!");
  println!("New string: {}", s);
}
