use regex::Regex;

fn main() {
    println!("Hello, world!");
    // Sum Regex
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    println!("Introduce an expression: ");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();
    let caps = re_add.captures(expression.as_str());

    let caps = caps.unwrap();

    println!("Result: {}", caps);
    
}
