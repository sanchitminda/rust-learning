// fn main() {
//     println!("Hello, world!");
// }
fn print_length(s: &String) {
    // Immutable borrow: Can read the data, but cannot change it.
    println!("Length: {}", s.len());
} // The immutable borrow ends here.

fn change_string(s: &mut String) {
    // Mutable borrow: Can change the data.
    s.push_str(" world!");
} // The mutable borrow ends here.

fn main() {
    let mut greeting = String::from("hello"); // `greeting` is the owner

    // 1. Immutable borrow (Reader) - OK
    print_length(&greeting);

    // 2. Mutable borrow (Writer) - OK
    change_string(&mut greeting);

    // 3. Attempting to use the original variable after a transfer (Move) - ERROR (if this was a function returning ownership)
    // println!("{}", greeting); // This would be an error if ownership was moved!

    // The key safety check: You cannot have two active mutable borrows (writers)
    // or a mutable and an immutable borrow (writer and reader) at the same time.
    // Rust forces the "write" to finish before another read or write can start.

    // Example of a disallowed scenario (will not compile):
    // let r1 = &greeting;          // Immutable read 1
    // let r2 = &mut greeting;      // Mutable write 1 (ERROR: Cannot borrow `greeting` as mutable because it is also borrowed as immutable)
}
