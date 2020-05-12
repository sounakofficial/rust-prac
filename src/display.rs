pub fn display(){
    //importing fmt
    use std::fmt;

    //deriving debug for structures std::fmt::Debug
    #[derive(Debug)]
    struct Structure(i32,i32);
    
    //implementing Display for Structure
    impl fmt::Display for Structure {
        fn fmt(&self , f: &mut fmt::Formatter) -> fmt::Result {
            //to return dont give ';' at end
            writeln!(f,"{},{}",self.0,self.1)
        }        
    }
    let minmax = Structure(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);
}