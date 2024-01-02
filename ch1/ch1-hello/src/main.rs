fn great_world() {
  println!("Hello, world!");

  let souther_germany = "Grüß Gott!";
  let japan = "ハロー・ワールド";

  let regions = [souther_germany, japan];

  for region in regions.iter() {
    println!("{}", &region);
  }
}

fn main() {
  great_world();
}
