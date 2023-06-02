fn main() {
  println!("{} days later", 28);
  
  println!("Hi {0}, this is {1} and Hello {1} this is {0}", "Alice", "Bob");

  println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

  println!("Base 10 : {}", 69240);
  println!("Base 2(binary) : {:b}", 69240);
  println!("Base 8(octal) : {:o}", 69240);
  println!("Base 16(hexadecimal) : {:x}", 69240);
  println!("Base 16(Hexadecimal) : {:X}", 69240);

}
