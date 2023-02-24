mod db;
use db::DB;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let database = DB.init();
        println!("i am happy!")
    }
}