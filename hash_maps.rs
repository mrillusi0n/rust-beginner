use std::collections::HashMap;

fn main() {
    let mut light = HashMap::new();    

    light.insert("name", String::from("Light Yagami"));

    for (k, v) in &light {
        println!("{}: {}", k, v);
    }
}
