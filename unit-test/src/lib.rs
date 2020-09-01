#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert() {
        assert!(1 + 1 == 2);
        assert!(1 + 1 == 3, "custom error msg");
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        assert_eq!(2 + 2, 4, "custom error msg");

        assert_ne!(2 + 1, 4);
        assert_ne!(2 + 1, 4, "custom error msg");

    }

    #[test]
    #[should_panic]
    fn should_panic() {
        panic!("should panic.");
    }

    #[test]
    #[should_panic(expected = "should panic.")]
    fn should_panic_expected() {
        panic!("should panic.");
    }

    // 使用Result<T, E>编写测试
    #[test]
    fn it_works_result() -> Result<(), String> {
        if 2 + 2 = 4 {
            Ok(())
        } else {
            Err(String::from("error msg"))
        }
    }

}
