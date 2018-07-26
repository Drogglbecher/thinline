/// Reprensents a parsed function argument.
#[derive(Default, Debug, Clone)]
pub struct Argument {
    name: String,
    atype: Option<String>,
}

impl Argument {
    pub fn new<S: Into<String>>(name: S, atype: Option<String>) -> Self {
        Argument {
            name: name.into(),
            atype: atype,
        }
    }
}
