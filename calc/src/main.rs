use std::io;

fn main() {
    let mut string: String =  String::new();

    println!("{:-^40}", "Calculdora por iteração");

    println!("Digite uma sequência de números separados por virgula.\n\
        Exemplo: 1,2,3,4..."
    );

    io::stdin()
        .read_line(&mut string)
        .expect("Error reading");

    let numbers: Vec<u32> = string.split(",")
        .map(|c| c.trim().parse().expect("Error parsing"))
        .collect();

    let result: u32 = numbers.iter().sum();

    println!("O resultado da soma é: {}", result);
}
