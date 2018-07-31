use error::*;

pub trait ParameterType: Default {
    fn parse()<T: ParameterType> -> Result<Parameter<T>>;<Paste>
}
