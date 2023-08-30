fn main() {
    // Embed the WIT folder into this application
    let resolve = include_wit::include_wit!("wit");
    
    // Print all interfaces in the resolve
    for x in &resolve.interfaces {
        println!("{x:?}");
    }
}