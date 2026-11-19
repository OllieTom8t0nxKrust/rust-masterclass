mod domain;
mod interface;

use std::{thread, time};

use domain::dominator;
use interface::smiley_face;

fn main() {
    let heart = "<3";
    let linguini = ":P";

    println!("Testing nested, chained, modules...");
    println!("Initiating Elegua programming luv...");
    thread::sleep(time::Duration::from_millis(2));

    dominator(heart);
    smiley_face(linguini);

    println!("END OF EXERCISE 1!")

}
