extern crate dictionary;

use dictionary::Dictionary;

fn main() {
  let mut dict = Dictionary::new();

  let x = dict.word();

  println!("{}", x);
}
