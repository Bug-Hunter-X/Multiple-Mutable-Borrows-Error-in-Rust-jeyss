fn main() {
    let mut x = 5;
    { //Creating a new scope
        let y = &mut x;
        *y += 1; 
    }
    { //Creating another new scope
        let z = &mut x; 
        *z += 2; 
    }
    println!("x = {}", x);
}