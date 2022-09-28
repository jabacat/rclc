pub fn list_ciphers() -> String {
    "None".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_ciphers_test() {
        assert_eq!(list_ciphers(), "caeser");
    }

    #[test]
    fn caeser_test() {
        assert_eq!(caeser("cat"), "zxq");
        assert_eq!(caeser("hello"), "eblln");
        assert_eq!(caeser("rclc"), "oziz");
        assert_eq!(caeser("RCLC"), "OZIZ");
    }

    #[test]
    fn shift_test() {
        assert_eq!(shift(3, "cat"), "zxq");
        assert_eq!(shift(3, "hello"), "eblln");
        assert_eq!(shift(3, "rclc"), "oziz");
        assert_eq!(shift(3, "RCLC"), "OZIZ");
        assert_eq!(shift(4, "rclc"), "nyhy");
        assert_eq!(shift(29, "rclc"), "oziz");
        assert_eq!(shift(29, "RCLC"), "OZIZ");
    }
}
