pub mod db;
use crate::db;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let database = DB{};
        println!("i am happy!")
    }
}