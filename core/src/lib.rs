/// Add two numbers together.
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke_test() {
        assert_eq!(add(2, 2), 2 + 2);
    }
}
