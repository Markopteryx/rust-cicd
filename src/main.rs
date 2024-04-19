fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let result = add_two_numbers(2, 3);
    println!("The result of adding 2 and 3 is: {}", result);
}

#[cfg(test)]
mod tests {
    use super::add_two_numbers;

    #[test]
    fn test_add_two_numbers() {
        assert_eq!(add_two_numbers(2, 3), 5);
    }
}
