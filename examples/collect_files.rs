extern crate thinlinelib;

use thinlinelib::Thinline;
use thinlinelib::language_type::C;

fn main() {
    let mut tl: Thinline<C> = Thinline::new();
    match tl.analyze_project("examples/c_project") {
        Ok(_) => {
            println!("Analyzed project successfully.");
        }
        Err(e) => println!("{}", e.to_string()),
    }
}
