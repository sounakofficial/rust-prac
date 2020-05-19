pub fn slice(){
    let arr: [i32; 6] = [1,2,3,4,5,6];

    //slicing the whole array
    let a: &[i32] = &arr;
    let b = &arr[..];


    println!("{:?}",a);
    println!("{:?}",b);
}