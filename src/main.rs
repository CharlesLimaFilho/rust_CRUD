use std::fs::File;
use std::io::{self, Read, Write};
use io::stdin;

//mod person;

fn main() {
    let file = File::open("");
}



/*
fn data_for_person() {
    let mut name = String::new();
    let mut cpf = String::new();
    let mut endereco = String::new();
    let mut salario = String::new();
    let mut genero = String::new();
    let mut nascimento = String::new();
    let mut departamento = String::new();

    println!("Digite o nome:");
    stdin().read_line(&mut name).unwrap();

    println!("Digite o CPF:");
    stdin().read_line(&mut cpf).unwrap();

    println!("Digite o endereço:");
    stdin().read_line(&mut endereco).unwrap();

    println!("Digite o salário:");
    stdin().read_line(&mut salario).unwrap();

    println!("Digite o gênero:");
    stdin().read_line(&mut genero).unwrap();

    println!("Digite a data de nascimento:");
    stdin().read_line(&mut nascimento).unwrap();

    println!("Digite o departamento:");
    stdin().read_line(&mut departamento).unwrap();
}
*/