fn main() {
    let mut x = 5;
    { //Creating a new scope to limit the lifetime of mutable reference y
        let y = &mut x;
        *y += 1;
    }
    let z = &x; // Now it's safe to create an immutable reference
    println!("x = {}", x); // x is now 6
    println!("z = {}", *z); //This will work now
}
