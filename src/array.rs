pub fn arr(){
    let arr1 : [ i8; 4] = [1, 2, 3, 4];
    let arr2 : [i8; 0] = [];
    let mut arr3 : [i8; 4] = [ 1,2,3,4];
    arr3[0] = 5;
    arr3[1] = 6;
    arr3[2] = 7;
    arr3[3] = 8;

    let arr4 = [0;4];
    let arr5 = ["x";4];


    println!("{:?}",arr1);
    println!("{:?}",arr2);
    println!("{:?}",arr3);
    println!("{:?}",arr4);
    println!("{:?}",arr5);
}