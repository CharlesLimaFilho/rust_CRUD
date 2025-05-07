use std::fs::rename;
use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Write}; // Write has write_all()
use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;

fn main() -> io::Result<()> {
    let mut option: &str;
    let mut buffer: String = String::new();
    let caminho: String = "output.txt".to_string();
    let array = ["CPF", "Nome", "Endereço", "Salário", "Sexo", "Data de nascimento", "Departamento", "Projetos"];

    loop {
        buffer.clear();
        println!("Escolha uma opção:");
        println!("1. Adicionar pessoa");
        println!("2. Buscar pessoa");
        println!("3. Alterar dados");
        println!("4. Ler arquivo");
        println!("5. Remover pessoa");
        println!("0. Sair");
        stdout().flush()?;
        stdin().read_line(&mut buffer)?;
        option = buffer.trim();

        match option {
            "1" => {
                buffer.clear();
                let mut dados: Vec<String> = Vec::new();

                for i in 0..array.len() {
                    buffer.clear();
                    println!("Digite o {}: ", array[i]);
                    stdout().flush()?;
                    stdin().read_line(&mut buffer)?;
                    dados.push(buffer.trim().to_string());
                }
                let _ = add_person(caminho.to_string(), dados);
            }
            "2" => {
                buffer.clear();
                println!("Digite o CPF: ");
                stdout().flush()?;
                stdin().read_line(&mut buffer)?;
                let cpf: String = buffer.trim().to_string();
                find_person(caminho.to_string(), cpf)?;
            }
            "3" => {
                let mut dados: Vec<String> = Vec::new();
                println!("\nCaso haja dados que não deseja alterar, digite -1\n");
                
                for i in 0..array.len() {
                    buffer.clear();
                    println!("Digite o {}: ", array[i]);
                    stdout().flush()?;
                    stdin().read_line(&mut buffer)?;
                    dados.push(buffer.trim().to_string());
                }
                update_data(caminho.to_string(), dados, array)?;
            }
            "4" => {
                print_file(caminho.to_string())?;
            }
            "5"=> {
                buffer.clear();
                println!("Digite o CPF: ");
                stdout().flush()?;
                stdin().read_line(&mut buffer)?;
                let cpf: String = buffer.trim().to_string();
                remove_person(caminho.to_string(), cpf)?;
            }
            "0" => {
                println!("Saindo...");
                break;
            }
            _ => {
                println!("Opção inválida, tente novamente.");
            }
        }
    }
    Ok(())
}

// A funcão add_person adiciona uma nova pessoa ao arquivo
// O arquivo é criado caso não exista
// O arquivo é aberto em modo append, ou seja, o novo conteúdo é adicionado ao final do arquivo
// A função recebe o caminho do arquivo e os dados da pessoa como parâmetros
// Os dados são passados como um vetor de strings
// A função formata os dados em uma string e escreve no arquivo

fn add_person(caminho: String, dados: Vec<String>) -> io::Result<()> {
    let mut file: File = OpenOptions::new().create(true).append(true).open(&caminho)?;
    


    let message: String = format!("{{\n\tCPF: {{ {} }}\n\tNome: {{ {} }}\n\tEndereço: {{ {} }}\n", dados[0], dados[1], dados[2]);
    let message: String = format!("{}\tSalário: {{ R$ {} }}\n\tSexo: {{ {} }}\n\tData de Nascimento: {{ {} }}\n", message, dados[3], dados[4], dados[5]);
    let message: String = format!("{}\tDepartamento: {{ {} }}\n\tProjetos: {{ {} }}\n}}\n", message, dados[6], dados[7]);


    file.write_all(message.as_bytes())?;
    println!("Pessoa adicionada.\n");
    Ok(())
}

// A função find_person busca uma pessoa no arquivo
// A função recebe o caminho do arquivo e o CPF da pessoa como parâmetros
// A função lê o arquivo linha por linha e verifica se a linha contém o CPF
// Se a linha contém o CPF, a função imprime a linha e as linhas seguintes até encontrar o fechamento do bloco

fn find_person(caminho: String, cpf: String) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(caminho)?;
    let reader = BufReader::new(file);
    let mut cond: bool = false;

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.trim();
                if l.contains(cpf.as_str()) || cond {
                    if l == "}" {
                        println!("");
                        return Ok(());
                    }

                    if !cond { println!("") };
                    println!("{}", l);
                    cond = true;
                }
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
                return Err(e);
            }
        }
    }
    println!("Não encontrado");
    return Ok(());
}

// A função print_file lê o arquivo e imprime seu conteúdo
// A função recebe o caminho do arquivo como parâmetro
// A função lê o arquivo linha por linha e imprime cada linha

fn print_file(caminho: String) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(caminho)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.trim();
                if l == "}" || l == "{" {
                    println!("");
                } else {
                    println!("{}", l);
                }
                
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    Ok(())
}


// Vector de strings para armazenar as informações, caso info "-1" não alterar
// A função update_data atualiza os dados de uma pessoa no arquivo
// A função recebe o caminho do arquivo, os novos dados e um vetor de strings com os nomes dos campos
// A função lê o arquivo linha por linha, enquanto escreve os dados num arquivo temporário
// e verifica se a linha contém o CPF
// Se a linha contém o CPF, a função atualiza os dados da pessoa
// Após encontrar o CPF, ele escreve as novas informações no arquivo temporário
// Após isso, a função renomeia o arquivo temporário para o nome do arquivo original

fn update_data(caminho: String, novas_info: Vec<String>, texto: [&'static str; 8]) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(&caminho)?;
    let mut temp_file: File = OpenOptions::new().write(true).create(true).open("output.tmp")?;
    let reader = BufReader::new(file);
    let mut message: String = String::new();
    let mut deve_alterar: bool = false;
    let mut cont = 0;

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.to_string();
                if l.contains(novas_info[0].as_str()) {
                    deve_alterar = true;
                    cont = 0;
                }
                if deve_alterar && (l == "}" || novas_info[cont] != "-1") {
                    if l == "}" {
                        temp_file.write_all(l.as_bytes())?;
                        temp_file.write_all(b"\n")?;
                        println!("");
                        deve_alterar = false;
                        cont = 0;
                    } else {
                        if l.contains("Salário:") {
                            message = format!("\t{}: {{ R$ {} }}\n", texto[cont], novas_info[cont]);
                        } else {
                            message = format!("\t{}: {{ {} }}\n", texto[cont], novas_info[cont]);
                        }
                        temp_file.write_all(message.as_bytes())?;
                        cont += 1;
                    }
                } else {
                    temp_file.write_all(l.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                    cont += 1;
                }
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    rename("output.tmp", caminho)?;
    Ok(())

}

// A função remove_person remove uma pessoa do arquivo
// A função recebe o caminho do arquivo e o CPF da pessoa como parâmetros
// A função lê o arquivo linha por linha e verifica se a linha contém o CPF
// Se a linha contém o CPF, a função não escreve a linha no arquivo temporário
// O "hold" é usado para evitar que a chave "{" seja escrita no arquivo temporário
// A função escreve as linhas restantes no arquivo temporário
// Após isso, a função renomeia o arquivo temporário para o nome do arquivo original

fn remove_person(caminho: String, cpf: String) -> io::Result<()> {
    let file: File = OpenOptions::new().read(true).open(&caminho)?;
    let mut temp_file: File = OpenOptions::new().write(true).create(true).open("output.tmp")?;
    let reader = BufReader::new(file);
    let mut deve_remover: bool = false;
    let mut hold: String = String::new();

    for line in reader.lines() {
        match line {
            Ok(_l) => {
                let l = _l.to_string();
                if l == "{" && hold == "" {
                    hold = l.clone();
                } else if l.contains(cpf.as_str()) {
                    deve_remover = true;
                } else if deve_remover && l == "}" {
                    deve_remover = false;
                    hold = String::new();
                } else if !deve_remover && hold != "" {
                    temp_file.write_all(hold.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                    hold = String::new();
                    temp_file.write_all(l.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                } else if !deve_remover {
                    hold = String::new();
                    temp_file.write_all(l.as_bytes())?;
                    temp_file.write_all(b"\n")?;
                }
            } 
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    }
    rename("output.tmp", caminho)?;
    Ok(())
}