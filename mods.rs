mod vectors;

use vectors::DeathNote;

fn main() {
    let mut rem_dn = DeathNote {
        names: Vec::new(),
        owner_name: String::from("Rem"),
    };

    rem_dn.names.push(String::from("L"));

    println!("{:#?}", rem_dn);
}
