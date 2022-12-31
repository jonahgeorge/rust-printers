extern crate printers;

fn main() {
    // Vector of system printers
    let printers = printers::get_printers();

    // Print directly in all printers
    for printer in printers {
        println!("{:?}", printer);
    }
}
