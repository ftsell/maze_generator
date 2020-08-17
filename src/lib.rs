#[macro_use]
pub mod prelude;
pub mod recursive_backtracking;

#[macro_use]
extern crate bitflags;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
