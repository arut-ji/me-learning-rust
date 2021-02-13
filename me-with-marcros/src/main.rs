use std::collections::HashMap;

#[allow(unused_macros)]
macro_rules! hey {
    ($name:expr) => {
        println!("Hey {}", $name);
    }
}

macro_rules! map {
    ($( $key: expr => $value: expr), *) => {{
        let mut hm = HashMap::new();
        $(hm.insert($key,$value);)*
        hm
    }};
}


fn main() {
    let m = map!("meow" => "hello", "meo" => "hello");
    println!("{:?}", m);
}
