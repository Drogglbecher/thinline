extern crate thinlinelib;

use thinlinelib::filter::*;

#[test]
fn blacklist_filter() {
    let blacklist_filter = [String::from("class3"), String::from("class4")];
    let filter = Filter::new(&blacklist_filter, &[]);
    assert_eq!(filter.filter_type, FilterType::Blacklist);
    assert_eq!(
        filter.filter,
        vec![String::from("class3"), String::from("class4")]
    );
    assert_eq!(filter.filter("class3"), false);
    assert_eq!(filter.filter("class1"), true);
}

#[test]
fn whitelist_filter() {
    let whitelist_filter = [String::from("class1"), String::from("class2")];
    let filter = Filter::new(&[], &whitelist_filter);
    assert_eq!(filter.filter_type, FilterType::Whitelist);
    assert_eq!(
        filter.filter,
        vec![String::from("class1"), String::from("class2")]
    );
    assert_eq!(filter.filter("class3"), false);
    assert_eq!(filter.filter("class1"), true);
}

#[test]
fn whitelist_and_blacklist_filter() {
    let whitelist_filter = [String::from("class1"), String::from("class2")];
    let blacklist_filter = [String::from("class3"), String::from("class4")];
    let filter = Filter::new(&blacklist_filter, &whitelist_filter);
    assert_eq!(filter.filter_type, FilterType::Whitelist);
    assert_eq!(
        filter.filter,
        vec![String::from("class1"), String::from("class2")]
    );
}

#[test]
fn empty_filter() {
    let filter = Filter::new(&[], &[]);
    assert_eq!(filter.filter_type, FilterType::NoType);
    assert_eq!(filter.filter("class1"), true);
}
