use regex::Regex;

fn make_operation(reg: Regex, mut expression: String, operation: &str) -> String {
    if operation.is_empty() {
        return "".to_string();
    }
    loop {
        let caps = reg.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();

        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();

        let result = match operation {
            "+" => left_value + right_value,
            "-" => left_value - right_value,
            "*" => left_value * right_value,
            "/" => left_value / right_value,
            _ => 0,
        };

        expression = expression.replace(cap_expression, &result.to_string());
    }
    expression
}

fn main() {
    // Regex
    let regex_addition = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let regex_subtraction = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let regex_multiplication = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    let regex_division = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    
    // Capture expression
    println!("Introduce an expression: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    
    //Multiplicacion
    expression = make_operation(regex_multiplication, expression, "*");
    //Division
    expression = make_operation(regex_division, expression, "/");
    //Suma
    expression = make_operation(regex_addition, expression, "+");
    //Resta
    expression = make_operation(regex_subtraction, expression, "-");

    //Mostrar resultados
    println!("Resultados: {}", expression);    
    
}
