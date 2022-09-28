pub fn list_ciphers() -> String {
    "caeser, shift".to_string()
}

pub struct Encryption {}

pub trait Encrypt {
    fn caeser(&self, msg: String) -> String;
    fn shift(&self, k: u32, msg: String) -> String;
}

impl Encrypt for Encryption {
    fn caeser(&self, msg: String) -> String {
        self.shift(3, msg)
    }

    fn shift(&self, k: u32, msg: String) -> String {
        let mut result = String::new();
        for c in msg.chars() {
            let r = if c.is_uppercase() { 65 } else { 97 };
            result.push(char::from_u32((c as u32 + k - r) % 26 + r).expect("Bounds error"));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn list_ciphers_test() {
        assert_eq!(list_ciphers(), "caeser, shift");
    }

    #[test]
    fn caeser_test() {
        let ec = Encryption {};
        assert_eq!(ec.caeser("abc".to_string()), "def");
        assert_eq!(ec.caeser("cat".to_string()), "fdw");
        assert_eq!(ec.caeser("hello".to_string()), "khoor");
        assert_eq!(ec.caeser("rclc".to_string()), "ufof");
        assert_eq!(ec.caeser("RCLC".to_string()), "UFOF");
    }

    #[test]
    fn shift_test() {
        let ec = Encryption {};
        assert_eq!(ec.shift(3, "cat".to_string()), "fdw");
        assert_eq!(ec.shift(3, "hello".to_string()), "khoor");
        assert_eq!(ec.shift(3, "rclc".to_string()), "ufof");
        assert_eq!(ec.shift(3, "RCLC".to_string()), "UFOF");
        assert_eq!(ec.shift(4, "rclc".to_string()), "vgpg");
        assert_eq!(ec.shift(29, "rclc".to_string()), "ufof");
        assert_eq!(ec.shift(29, "RCLC".to_string()), "UFOF");
    }
}
