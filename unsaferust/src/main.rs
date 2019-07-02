// Things you can do
// 1. Dereference a raw pointer
// 2. Call an unsafe function or method
// 3. Access of modify a mutable static variable
// 4. Implement an unsafe trait

fn main() {
    // raw pointers
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
