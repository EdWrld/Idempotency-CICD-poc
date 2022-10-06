fn main() {
    println!("{}", hello_world());
    println!("{}", bad_hello_world());
}

fn hello_world() -> &'static str {
    let s = "Hello, world!";
    return s;
}

fn bad_hello_world() -> &'static str {
    let f = "Hello, fantasy";
    return f;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hello_world_success() {
        assert_eq!(hello_world(), "Hello, world!")
    }
    #[test]
    fn test_hello_world_bad() {
        // Should fail this is to test how the pipeline react
        assert_eq!(bad_hello_world(), "Hello, world!")
    }
}
