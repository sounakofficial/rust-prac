pub fn closure(){
    //Also known as anonymous functions or lambda functions.
    let x = 5;
    let x_closure_type1 = |i: i32| -> i32 { i * i };
    let x_closure_type2 = |i: i32| -> i32 { i * i }(x);
    let x_closure_type3 = |i| i * i;
    let x_closure_type4 = |i| -> i32{ i * i }(x);

    // return types are mandatory when calling
    println!("{}",x_closure_type1(x));
    println!("{}",x_closure_type2);
    println!("{}",x_closure_type3(x));
    println!("{}",x_closure_type4);
}