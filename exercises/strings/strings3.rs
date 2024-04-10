// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.



fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut start = 0;
    let mut end = input.len();
    for (index, &byte) in input.as_bytes().iter().enumerate(){
        if !byte.is_ascii_whitespace(){
            start = index;
            break;
        }
    }
    for (index, &byte) in input.as_bytes().iter().enumerate().rev(){
        if !byte.is_ascii_whitespace(){
            end = index + 1;
            break;
        }
    }
    let a  = &input[start..end];
    let b = a.to_string();
    b
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
   let mut a :String = input.to_string();
   a.push_str(" world!");
   a
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    let a: String = input.replace("cars", "balloons");
    a
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
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
