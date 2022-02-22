
/// String Interpolation 
fn main() {
    let greetings = "Hello";
    let subject = "World";
    let crash_reason="Server wanted a nap";
    println!("{}, {}!", greetings, subject);
    let tmp= format!("Hello, {}", subject);
    println!("{}",tmp);
    
    panic!("I crashed! {}", crash_reason);
    
    println!("This will never get run.");
}