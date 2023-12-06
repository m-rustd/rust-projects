use proc_builder::DarlingDerive;

#[derive(DarlingDerive)]
#[myderive(name = "foo", age = 12)]
struct Foo(u64);

fn main() {
    println!("Hello, world!");
}
