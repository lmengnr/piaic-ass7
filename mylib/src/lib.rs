#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod module3{
    pub mod sub_mod3{
        pub fn printQ3(){
            println!("Calling function for Q3 \nmade in library package mylib");
        }
    }
}
