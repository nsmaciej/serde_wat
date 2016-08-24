# Scary macros for serde_json

<center>[Documentation](http://mgoszcz2.github.io/serde_wat/serde_wat/index.html) | [Crate](https://crates.io/crates/serde_wat)</center>

![Wat](https://i.imgur.com/IppKJ.jpg)

Macros for easily accessing `serde_json`
[Value](https://docs.serde.rs/serde_json/value/enum.Value.html)s
in JavaScript like fashion

```toml
# Cargo.toml
[dependencies]
serde_wat = "0.1"
```

## Usage

```rust
#[macro_use] extern crate serde_wat;
extern crate serde_json;

use serde_json::from_str;
use serde_json::value::Value;

let a: Value = from_str(r#"{"b": {"c": 42}}"#).unwrap();
// Access unwrapping
assert_eq!(wat!(a.b.c as i64), 42);
// Safe access
assert_eq!(wat!(a.b as &str?), None);
// Testing
assert_eq!(wat!(a is bool), false);
```

## Examples

```rust
wat!(a.b.c as i64);
wat!(a as &mut Vec?);
wat!(a as &str);
wat!(a.b is i64);
wat!(a.b is &Map?);
wat!(a is &Map);
wat!(a.0 as bool);
```
