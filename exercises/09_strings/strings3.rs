fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    // trip start space
    let mut l = 0;
    let mut r  = input.len() - 1;
    // Find first non-whitespace character
    for (i, c) in input.char_indices() {
        if !c.is_whitespace() {
            l = i;
            break;
        }
    }

    // Find last non-whitespace character
    for (i, c) in input.char_indices().rev() {
        if !c.is_whitespace() {
            // Add 1 to include this character (end is exclusive)
            r = i + c.len_utf8();
            break;
        }
    }

    // Return the slice from start to end
    // If string is all whitespace, start will be > end
    if l >= r {
        return "";
    }
    &input[l..r]

}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut owned_str = input.to_owned();
    owned_str.push_str(" world!");
    owned_str
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    input.replace("cars", "balloons")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
