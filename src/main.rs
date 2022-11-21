fn main() {

    let message = std::env::args().nth(1)
    .expect("missing the message. Usage: new message <message>");

    println!("\n");
    print!("< {} >", message);
    println!("\n");
    println!("   _______");
    println!("     \\");
    println!("      \\");
    println!("       _______");
    println!("      /_______\\ ");
    println!("      |       |");
    println!("     (|  O.0  |)");
    println!("      \\  __  /");
    println!("       \\____/ ");
}
