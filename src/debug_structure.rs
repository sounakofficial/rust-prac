pub fn debug_structure(){
    //implementint std::fmt::Debug
    #[derive(Debug)]
    struct Structure(i32);
    println!("{:?}",Structure(5));
    
    //pretty printing using {:#?}
    println!("pretty printed : {:#?}",Structure(5));
}