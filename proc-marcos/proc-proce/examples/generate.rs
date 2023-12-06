pub mod generated {
    use proc_proce::generate;

    generate!("proc-proce/fixtures/person.json");
}

use generated::*;

fn main() {
    let person = Person {
        first_name: "mz".into(),
        last_name: "mister".into(),
        skill: Skill {
            name: "Rust".into(),
        },
    };
    println!("{:#?}", person);
}