pub fn balanced(input: &str) -> bool {
    let mut stack = Vec::new();

    for c in input.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' if stack.pop().filter(|top| *top == '(').is_none() => {
                return false;
            }
            ']' if stack.pop().filter(|top| *top == '[').is_none() => {
                return false;
            }

            '}' if stack.pop().filter(|top| *top == '{').is_none() => {
                return false;
            }
            _ => continue,
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(balanced(""), true);
        assert_eq!(balanced("()"), true);
        assert_eq!(balanced("(())"), true);
        assert_eq!(balanced("([(abc)])"), true);
        assert_eq!(balanced("()[]({})"), true);
        assert_eq!(balanced("("), false);
        assert_eq!(balanced(")"), false);
        assert_eq!(balanced("["), false);
        assert_eq!(balanced("]"), false);
        assert_eq!(balanced("{"), false);
        assert_eq!(balanced("}"), false);
        assert_eq!(balanced("(}"), false);
        assert_eq!(balanced("(i]"), false);
        assert_eq!(balanced("([]"), false);
        assert_eq!(balanced("]["), false);
        assert_eq!(balanced("( { [ } { ] } )"), false);
    }
}
