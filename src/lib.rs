#![cfg_attr(not(any(test, feature = "use-std")), no_std)]

pub mod de;
pub mod error;
pub mod types;

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
