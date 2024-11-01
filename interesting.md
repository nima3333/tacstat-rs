# Interesting stuffs about Rust

- Use of regex

``` rust
    let object_creation_pattern = Regex::new(r"^....").unwrap();
    if let Some(caps) = object_creation_pattern.captures(line)  {
        hm.insert(caps[1].to_string().unwrap()...);
    }
```

- Lifetime-BC for caps

Caps reference lifetime not enough for hashmap
to_string -> create a new string on heap, will have the lifetime of the hashmap 

- Different style of unwrap

``` rust
//1. Unwrap -> panic if result in error
let key = caps[1].to_string().unwrap();

//2. Error message
let key = i32::from_str_radix(&caps[1], 16).expect("Failed to parse key");

//3. Matching result
match i32::from_str_radix(&caps[1], 16) {
    Ok(key) => {
        hm.insert(key, value);
    }
    Err(e) => {
        eprintln!("Error parsing key: {}", e);
    }
}

//4. Propagate error ?
let value = i32::from_str_radix(hex_str, 16)?;
```

- f64 implements copy traits

Then, when inserting a mutable f64 inside a f64 hashmap, it is copied