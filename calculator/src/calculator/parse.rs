pub struct Expression {
    pub numbers: Vec<f64>,
    pub operators: Vec<char>,
}
pub fn parse(input: &str) -> Result<Expression, String> {
    let available_operators = ['+', '-'];

    let input = input.trim();

    let mut numbers: Vec<f64> = Vec::new();
    let mut operators = Vec::new();

    let mut current_number = String::new();

    for char in input.chars() {
        if char.is_digit(10) {
            current_number.push(char);
        } else if char == '.' {
            if !current_number.contains('.') {
                current_number.push(char);
            } else {
                return Err("Number cant contain more then one dot!".to_string());
            }
        } else if available_operators.contains(&char) {
            operators.push(char);

            if !current_number.is_empty() {
                match current_number.parse::<f64>() {
                    Ok(num) => numbers.push(num),
                    Err(_e) => println!("error during parsing a num"),
                }
                current_number.clear();
            }
        } else if char.is_whitespace() {
            continue;
        } else {
            return Err(format!("Unexpected char {char}"));
        }
    }
    if !current_number.is_empty() {
        match current_number.parse::<f64>() {
            Ok(num) => numbers.push(num),
            Err(_e) => println!("error during parsing a num"),
        }
    }
    if numbers.len() != operators.len() + 1 {
        return Err(
            "Некорректное выражение: неверное количество чисел и операторов"
                .parse()
                .unwrap(),
        );
    }
    return Ok(Expression { numbers, operators });
}
