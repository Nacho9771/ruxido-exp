fn main() {
    // Suffixed literals, their types are known at initialization
    let a = 1u8;
    let b = 2u32;
    let c = 3f32;
    let d = 4u32;
    let e = 5u32;
    let z = 15u32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `a` in bytes: {}", std::mem::size_of_val(&a));
    println!("size of `b` in bytes: {}", std::mem::size_of_val(&b));
    println!("size of `c` in bytes: {}", std::mem::size_of_val(&c));
    println!("size of `d` in bytes: {}", std::mem::size_of_val(&d));
    println!("size of `e` in bytes: {}", std::mem::size_of_val(&e));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}