
pub mod second_layer;

pub fn print_something() {
    println!("external mod is printed");
    second_layer::more_printing();
}