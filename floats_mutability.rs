//Floats

fn main(){
    let x=1.1;
    let y=2.2;
    println!("x times y is {}", x*y); // x times y is 2.4200000000000004
}

// main is a function 
// println is a macro , so sorta what "!" 
//macro: works bit like a function call but rather than calling a function it's sorta doing some 
// code transformation instead 
//so what println is actually doing is it's taking these argruments we're givinh it 
// and its going to translate them at compile time into some number of other function calls,
// like for example, stapling together these strings, taking result of this multiplication and then 
// turning that into a string and then concatening it with a string and so forth 
// string interpolation also works with floats
// binary floats are very fast so if u wanted to give this expectd answer that we would expect 
// from normal math that would be a significant performance cost because mordern hardware 
// can do this in one instruction whereas if you want to do decimal math, it needs a lot more overhead 
//than binary math does like this when it comes to floating point numbers 
// read about macros online 

//Mutability

let x=1.1; // both const and immutable 

    x=2.2; // error: cannot assign twice to immutable variable 'x'
    
let mut x=1.1; // neither const not immutable 
                // making this binding mutable : `mut x `
        x=2.2; 


