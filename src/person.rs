struct Person {
    name: String,
    cpf: String,
    endereco: String,
    salario: f32,
    genero: String,
    nascimento: String,
    departamento: String,
    projetos: Vec<String>,
}

pub fn create_person(
    name: String,
    cpf: String,
    endereco: String,
    salario: f32,
    genero: String,
    nascimento: String,
    departamento: String,
) -> Person {
    return Person {
        name,
        cpf,
        endereco,
        salario,
        genero,
        nascimento,
        departamento,
        projetos: Vec::new(),
    }
}