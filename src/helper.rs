#![macro_use]

#[macro_export]
macro_rules! unwrap_or_continue {
    ($e:expr) => {
        match $e {
            Some(e) => e,
            None => continue,
        }
    };
}
