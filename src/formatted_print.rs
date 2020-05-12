pub fn formatted_print(){
    //general
    println!("normal string");

    //with placeholders
    println!("{} is a number, {} is string and {} is boolean,",3.14,"Alice",true);

    //with positional arguments
    println!("all {0} are {1} but all {1} aren't {0}","thumb","fingers");
    
    //with named arguments
    println!("all {arg1} are {arg2} but all {arg2} aren't {arg1}",arg1="thumb",arg2="fingers");

    //with special formatting
    println!("{0} - integer, {0:b} - binary, {0:x} - hex, {0:o} - octal, {0:e} - exponential",10);
}