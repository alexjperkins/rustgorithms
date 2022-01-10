pub fn is_valid(code: &str) -> bool {
    // Exit early if string is of length 1 or less.
    if code.len() < 2 {
        return false;
    }

    // Remove all whitespace & reverse.
    let mut code: Vec<char> = code
        .chars()
        .filter(|c| c.is_digit(10))
        .rev()
        .collect();

    // Iterate through every second item, double, if greater than 9 add the digits.
    for (i, c) in code.clone().iter().enumerate() {
        if i % 2 == 0 {
            continue
        }

        let c: u32 = c.to_digit(10).unwrap_or(0) * 2;
        if c < 9 {
            code[i] = std::char::from_u32(c).unwrap();
            continue
        }

        let c = {
            c
                .to_string()
                .chars()
                .map(|a| a.to_digit(10).unwrap_or(0))
                .reduce(|a, b| a + b)
                .unwrap()
        };

        code[i] = std::char::from_u32(c).unwrap();
    }

    // Return true if modulo 10 of sum of all chars is zero, otherwise return false.
    (code.iter().map(|a| a.to_digit(10).unwrap_or(0)).reduce(|a, b| a + b).unwrap() % 10) == 0
}
