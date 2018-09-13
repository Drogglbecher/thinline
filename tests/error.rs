extern crate thinlinelib;

use std::io;
use thinlinelib::error::*;

static ERROR_STR: &str = "Something went wrong here.";

#[test]
fn io_error_to_thinline_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, ERROR_STR);
    let wiki_error: Error = io_error.into();
    assert_eq!(wiki_error.description(), ERROR_STR.to_string());
}
