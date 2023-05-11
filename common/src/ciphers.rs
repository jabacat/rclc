pub fn list_ciphers() -> String {
    "caesar, shift".to_string()
}

pub struct Encryption {}
pub struct Decryption {}

pub trait Encrypt {
    fn caesar(&self, msg: String) -> String;
    fn shift(&self, k: u32, msg: String) -> String;
}

impl Encrypt for Encryption {
    fn caesar(&self, msg: String) -> String {
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

pub trait Decrypt {
    fn caesar(&self, msg: String) -> String;
    fn shift(&self, k: u32, msg: String) -> String;
}

impl Decrypt for Decryption {
    fn caesar(&self, msg: String) -> String {
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
        assert_eq!(list_ciphers(), "caesar, shift");
    }

    #[test]
    fn caesar_encrypt_test() {
        let ec = Encryption {};
        assert_eq!(ec.caesar("abc".to_string()), "def");
        assert_eq!(ec.caesar("cat".to_string()), "fdw");
        assert_eq!(ec.caesar("hello".to_string()), "khoor");
        assert_eq!(ec.caesar("rclc".to_string()), "ufof");
        assert_eq!(ec.caesar("RCLC".to_string()), "UFOF");
    }

    #[test]
    fn shift_encrypt_test() {
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
