extern crate printers;

fn main() {
    let printer = printers::get_printer_by_name("NPI642118 (HP LaserJet M110w)")
        .expect("Failed to find printer");

    let job = printers::print(&printer, "Hello World".as_bytes());
    println!("{:?}", job);
}
