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

// benifits of macros over functions
// number of arguments are fixed in fn 
// macro have no limit on arguments 
// macros are sort of strictly more powerfull than functions in every way
// expect on which is that fn you're allowed to pass around ,
// like you can have first-class functions in rust , 
// you cannot pass macros 
// performance difference between macros vs fn => yes at compile time not at runtime 
// because macros has to do that expansion work at compile time, 
// macros are one of the things that can make your compile times longer
// in rust  depending on the complexity of macro and how many you're using so forth



