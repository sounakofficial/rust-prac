pub fn vec_disp(){
    use std::fmt;

    //only to show debug info uncomment underline
    //#[derive(Debug)] 
    //creating a list or vector of i32
    struct List(Vec<i32>);

    impl fmt::Display for List{
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            //getting the starting of the vector
            let vec = &self.0;
            // the '?' sign after write! is for multiple fmt::Result
            write!(f, "[")?;
            // the count starts from 0 and iterates and then pulls 
            // the value from vector 'vec' and index 'count'; stores it in 'v'
            for (count, v) in vec.iter().enumerate() {
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}", v)?;
            }
            write!(f,"]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);
    //for debugging uncomment underline
    //println!("{:?}", v);
}