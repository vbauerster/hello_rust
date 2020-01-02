pub fn rot13(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            'A'..='M' | 'a'..='m' => (c as u8 + 13) as char,
            'N'..='Z' | 'n'..='z' => (c as u8 - 13) as char,
            _ => c,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode() {
        assert_eq!(rot13("Lbh penpxrq gur pbqr!"), "You cracked the code!");
    }

    #[test]
    fn encode() {
        assert_eq!(rot13("You cracked the code!"), "Lbh penpxrq gur pbqr!");
    }
}
