use yaml_rust::Yaml;

////////////////////////////////////////////////////////////////////////////////

pub trait Conversion {
    /// Consumes an Option<Vec<&'a str>> and returns it's elements as String.
    fn to_string_vec(self) -> Vec<String>;
}

impl<'a> Conversion for Option<Vec<&'a str>> {
    fn to_string_vec(self) -> Vec<String> {
        if let Some(vec) = self {
            vec.iter().map(|f| String::from(*f)).collect()
        } else {
            vec![]
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait ValueParser {
    /// Reads a bool at the given config path.
    fn get_bool(&self, keys: &[&str], default: bool) -> bool;

    /// Reads a string at the given config path.
    fn get_str(&self, keys: &[&str]) -> Option<&str>;

    /// Reads a string vector at the given config path.
    fn get_str_vec(&self, keys: &[&str]) -> Option<Vec<&str>>;
}

impl ValueParser for Yaml {
    fn get_bool(&self, keys: &[&str], default: bool) -> bool {
        let mut yml_obj = self;

        for key in keys {
            if yml_obj[*key].is_badvalue() {
                return default;
            }

            yml_obj = &yml_obj[*key];
            if let Some(yml_bool) = yml_obj.as_bool() {
                return yml_bool;
            }

            if yml_obj[0].is_badvalue() {
                continue;
            }
        }

        default
    }

    fn get_str(&self, keys: &[&str]) -> Option<&str> {
        let mut yml_obj = self;

        for key in keys {
            if yml_obj[*key].is_badvalue() {
                return None;
            }

            yml_obj = &yml_obj[*key];
            if let Some(yml_str) = yml_obj.as_str() {
                return Some(yml_str);
            }

            if yml_obj[0].is_badvalue() {
                continue;
            }

            if let Some(yml_str) = yml_obj[0].as_str() {
                return Some(yml_str);
            }
        }

        None
    }

    fn get_str_vec(&self, keys: &[&str]) -> Option<Vec<&str>> {
        let mut yml_obj = self;
        let mut yml_vec: Vec<&str> = Vec::new();

        for key in keys {
            if yml_obj[*key].is_badvalue() {
                return None;
            }

            yml_obj = &yml_obj[*key];
            let mut i = 0;
            while !yml_obj[i].is_badvalue() {
                if let Some(yml_str) = yml_obj[i].as_str() {
                    yml_vec.push(yml_str);
                }
                i += 1;
            }
        }

        if !yml_vec.is_empty() {
            return Some(yml_vec);
        }

        None
    }
}
