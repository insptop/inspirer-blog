#[macro_use]
extern crate sea_orm;
#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate serde;

pub mod app;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
