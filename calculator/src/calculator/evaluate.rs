pub fn evaluate(numbers: &Vec<f64>, operators: &Vec<char>) -> Result<f64, String> {
    let mut result = numbers[0];
    for index in 0..operators.len() {
        match operators[index] {
            '+' => result += &numbers[index + 1],
            '-' => result -= &numbers[index + 1],
            _ => return Err("Unexpected operator".parse().unwrap()),
        }
    }
    return Ok(result);
}
