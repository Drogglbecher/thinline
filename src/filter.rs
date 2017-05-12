#[derive(PartialEq, Debug)]
pub enum FilterType {
    NoType,
    Blacklist,
    Whitelist,
}

impl Default for FilterType {
    fn default() -> FilterType {
        FilterType::NoType
    }
}

#[derive(Default)]
pub struct Filter {
    pub filter: Vec<String>,
    pub filter_type: FilterType,
}

impl Filter {
    pub fn new(bl_filter: &[String], wl_filter: &[String]) -> Self {
        if !bl_filter.is_empty() && !wl_filter.is_empty() {
            warn!("Blacklist and whitelist filter set. Taking whitelist one.");
        }

        let (filter, filter_type) = if !wl_filter.is_empty() {
            (wl_filter.to_vec(), FilterType::Whitelist)
        } else if !bl_filter.is_empty() {
            (bl_filter.to_vec(), FilterType::Blacklist)
        } else {
            (Vec::new(), FilterType::NoType)
        };

        debug!("Setting up {:?} with args {:?}", filter_type, filter);

        Filter {
            filter: filter,
            filter_type: filter_type,
        }
    }

    pub fn filter(&self, child_name: &str) -> bool {
        if self.filter.is_empty() {
            return true;
        }

        (self.filter_type == FilterType::Blacklist &&
             !self.filter.contains(&String::from(child_name))) ||
            (self.filter_type == FilterType::Whitelist &&
                 self.filter.contains(&String::from(child_name)))
    }
}
