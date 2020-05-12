//hello_world.rs
mod hello_world; 
//formatted_print.rs
mod formatted_print;
//debug_structure.rs
mod debug_structure;
//display.rs
mod display;
//vector_display.rs
mod vector_display;
//float_formatting.rs
mod float_formatting;

fn main() {
    //hello_world.rs
    hello_world::hello_world();

    //formatted_print.rs
    formatted_print::formatted_print();

    //debug_structure.rs
    debug_structure::debug_structure();

    //display.rs
    display::display();

    //vector_display.rs
    vector_display::vec_disp();

    //float_formatting.rs
    float_formatting::float_format();
}
