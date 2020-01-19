mod lib;
use mylib;

pub mod module1{
    pub mod sub_mod1{
        pub fn printQ1(){
            println!("Calling function for Q3 \nmade in main.rs\n");
        }
    }
}


fn main() {

    module1::sub_mod1::printQ1();
    lib::module2::sub_mod2::printQ2();
    mylib::module3::sub_mod3::printQ3();
    
}
