#[derive(Debug)]
struct DeathNote {
    names: Vec<String>,
    owner_name: String,
}

fn main() {
    let mut ryuk_dn = DeathNote {
        names: Vec::new(),
        owner_name: String::from("Ryuk")
    };

    ryuk_dn.names.push(String::from("Light Yagami"));

    println!("{:#?}", ryuk_dn);
}
