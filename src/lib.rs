
pub fn add_one(x:i32) -> i32{
    x + 1
}

#[cfg(test)]
mod utests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
