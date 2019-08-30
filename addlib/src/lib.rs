#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_add() {
        assert_eq!(4, super::add(3, 1));
        assert_ne!(3, super::add(2, 2));
    }
}

/// Usage: calculate two integer's sum
pub fn add(a:i32, b:i32) -> i32 {
    a+b
}
