fn main() {
    println!("{}", greet("world"));
}

fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("world"), "Hello, world!");
        assert_eq!(greet("Rust"), "Hello, Rust!");
    }
}
