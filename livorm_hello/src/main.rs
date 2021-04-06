extern crate livorm;

use livorm::HelloLiv;
#[derive(HelloLiv)]
struct Hello;

fn main() {
    println!("Hello, world!");
    Hello::hello_liv();
    Hello::auto_hello();
}
