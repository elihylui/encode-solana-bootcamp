fn main() {
    println!("Hello, world!");

    let a = vec![1,2,3];
    println!("{:?}", a);
    let b = a;
    //println!("{:?}", a); //this won't work as a is not owning anymore
}
