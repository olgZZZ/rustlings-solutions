// tests2.rs
// This test has a problem with it -- make the test compile! Make the test
// pass! Make the test fail! Execute `rustlings hint tests2` for hints :)


fn hello(word: String) -> String {
    word.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn you_can_assert_eq() {
        assert_eq!(hello("Hello!".to_string()), "Hello!".to_string());
    }
}
