use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut operator = String::new();

    io::stdin().read_line(&mut input1).expect("failed to read number");
    io::stdin().read_line(&mut operator).expect("failed to read operator");
    io::stdin().read_line(&mut input2).expect("failed to read number");

    let number1: i32 = input1.trim().parse().expect("input1 was not a number");
    let number2: i32 = input2.trim().parse().expect("input2 was not a number");
    let operator = operator.trim();

    if operator == "+" {
        let answer = number1 + number2;
        println!("{}", answer);
    }
    if operator == "-" {
        let answer = number1 - number2;
        println!("{}", answer);
    }
    if operator == "*" {
        let answer = number1 * number2;
        println!("{}", answer);
    }
    if operator == "/" {
        let answer = number1 / number2;
        println!("{}", answer);
    }
}

//AI was used to convert from string to in, like why tf is 'z.trim().parse().expect("x was not a number")' how you convert from string to int