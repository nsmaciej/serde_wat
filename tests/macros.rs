#[macro_use]
extern crate serde_wat;
extern crate serde_json;

use serde_json::from_str;
use serde_json::value::Value;

#[test]
fn lookup() {
    let a: Value = from_str(r#"{"b": {"c": 42}}"#).unwrap();
    assert_eq!(wat!(a.b.c as i64), 42)
}

#[test]
fn lookup_mut() {
    let mut a: Value = from_str("[null]").unwrap();
    assert_eq!(wat!(a as &mut Vec), &mut [Value::Null]);
}

#[test]
fn cast() {
    let a: Value = from_str("\"\"").unwrap();
    assert_eq!(wat!(a as &str), "")
}

#[test]
fn check() {
    let a: Value = from_str(r#"{"b": 42}"#).unwrap();
    assert!(wat!(a.b is i64))
}

#[test]
fn check_map() {
    let a: Value = from_str(r#"{"b": {}}"#).unwrap();
    assert!(wat!(a.b is &Map))
}

#[test]
fn check_toplevel() {
    let a: Value = from_str(r#"{}"#).unwrap();
    assert!(wat!(a is &Map))
}

#[test]
fn lookup_index() {
    let a: Value = from_str("[true]").unwrap();
    assert!(wat!(a.0 as bool))
}

#[test]
fn safe_null_check() {
    let a: Value = from_str("{}").unwrap();
    assert_eq!(wat!(a.b is ()?), false)
}

#[test]
fn safe_null_get() {
    let a: Value = from_str("{}").unwrap();
    assert_eq!(wat!(a.b as ()?), None)
}

#[test]
fn safe_top_level_get() {
    let a: Value = from_str("{}").unwrap();
    assert_eq!(wat!(a as ()?), None)
}

#[test]
fn just_none() {
    let a: Value = from_str(r#"{"b": {"c": 42}}"#).unwrap();
    assert_eq!(wat!(a.b as &str?), None);
}
