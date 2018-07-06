extern crate thinlinelib;

use thinlinelib::Thinline;

fn main() {
    let mut tl = Thinline::new();
    match tl.analyze_project("examples/c_project") {
        Ok(_) => {
            println!("Analyzed project successfully.");
        }
        Err(e) => println!("{}", e.to_string()),
    }
}
