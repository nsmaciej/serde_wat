use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

macro_rules! generate_getters {
    ($fd:ident: $($x:ty => $y:ident ($z:ident)),+,) => {{
        $(
            generate_getter(&mut $fd, stringify!($x), stringify!($y));
            generate_checker(&mut $fd, stringify!($x), stringify!($z));
        )*
    }}
}

fn generate_getter(fd: &mut File, typ: &str, method: &str) {
    // Normal getter
    fd.write_fmt(format_args!("\n($var:tt.$($x:tt).* as {}) => {{
        wat!($var.$($x).*).as_{}().unwrap()
    }};\n", typ, method)).unwrap();

    // Normal safe getter
    fd.write_fmt(format_args!("\n($var:tt.$($x:tt).* as {}?) => {{
        wat!($var.$($x).*?).and_then(|x| x.as_{}())
    }};\n", typ, method)).unwrap();

    // Top level getter
    fd.write_fmt(format_args!("\n($var:tt as {}) => {{
        $var.as_{}().unwrap()
    }};\n", typ, method)).unwrap();

    // Top level safe getter
    fd.write_fmt(format_args!("\n($var:tt as {}?) => {{
        $var.as_{}()
    }};\n", typ, method)).unwrap();
}

fn generate_checker(fd: &mut File, typ: &str, method: &str) {
    // Normal check
    fd.write_fmt(format_args!("\n($var:tt.$($x:tt).* is {}) => {{
        wat!($var.$($x).*).is_{}()
    }};\n", typ, method)).unwrap();

    // Safe check
    fd.write_fmt(format_args!("\n($var:tt.$($x:tt).* is {}?) => {{
        wat!($var.$($x).*?).map_or(false, |x| x.is_{}())
    }};\n", typ, method)).unwrap();

    // Top level check
    fd.write_fmt(format_args!("\n($var:tt is {}) => {{
        $var.is_{}()
    }};\n", typ, method)).unwrap();
    // Top level safe check would be silly.. there are no keys to lookup
}

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("macros.rs");
    let mut fd = File::create(&dest_path).unwrap();
    fd.write_all(include_bytes!("header.txt")).unwrap();

    generate_getters! {fd:
        &Vec => array (array),
        i64 => i64 (number),
        u64 => u64 (number),
        i64 => i64 (number),
        &Map => object (object),
        bool => bool (boolean),
        &str => str (string),
        () => null (null),
    }
    generate_getter(&mut fd, "&mut Vec", "array_mut");
    generate_getter(&mut fd, "&mut Map", "object_mut");

    fd.write_all(b"}").unwrap();
}
