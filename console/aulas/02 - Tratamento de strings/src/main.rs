use std::io;

fn main() {
    // str é diferente de string
    let s = "Oláaa".to_string();
    let s = String::from("Olá");

    let s = String::new();
    let s = "".to_string();

    let mut z = String::new();
    z += "teste";

    println!("{}", s);
    println!("{}", z);

    // em dados primitivos do tipo inteiro é sempre bom especificar o bit deles
    let numero: i8 = 123;
    println!("{}", numero);
}
