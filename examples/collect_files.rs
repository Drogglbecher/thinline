extern crate thinlinelib;

use thinlinelib::Thinline;

fn main() {
    let mut tl = Thinline::new();
    match tl.collect_sources("examples/c_project", ".thinline.yml") {
        Ok(_) => {
            tl.extract_fct_symbols().unwrap();
            tl.reconstruct_fn().unwrap();
        }
        Err(e) => println!("{}", e.to_string()),
    }
}
