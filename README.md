# serde_wat

 Scary macros for serde_json

acros for easily accessing serde_json Values

``rust
[macro_use]
xtern crate serde_wat;
xtern crate serde_json;

se serde_json::from_str;
se serde_json::value::Value;

et a: Value = from_str(r#"{"b": {"c": 42}}"#).unwrap();
/ Access unwrapping
ssert_eq!(wat!(a.b.c as i64), 42);
/ Safe access
ssert_eq!(wat!(a.b as &str?), None);
/ Testing
ssert_eq!(wat!(a is bool), false);
``

## Examples

``rust
at!(a.b.c as i64);
at!(a as &mut Vec?);
at!(a as &str);
at!(a.b is i64);
at!(a.b is &Map?);
at!(a is &Map);
at!(a.0 as bool);
``
