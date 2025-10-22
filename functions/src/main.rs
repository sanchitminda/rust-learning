fn func(){
    println!("Hello func");
}
fn add(a: &mut, b: i32) -> i32{
    let a = a + b;
    return a;
}
fn main() {
    func();
    let x:i32 = 5;
    println!("Sum = {}",add(&x,2));
    println!("Sum = {}",add(&x,5));
}
