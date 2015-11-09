fn main() {
    println!("{}", generate_reference("Carlos Castro", "teste", "2015", "abc"));
    println!("{}", generate_reference("Carlos de Castro Silva", "teste", "2015", "abc"));
}

//SOBRENOME, Prenome. Título, ano. Disponível em: <url>. Acesso em: dia mês (abreviado) ano.
fn generate_reference(author: &str, title: &str, year: &str, link: &str) -> String {
    let mut author_parts: Vec<&str> = author.split_whitespace().collect();
    let mut author_surname: &str = match author_parts.pop() {
        Some(x) => x,
        None => panic!("Last name not found!"),
    };
    let author_surname: &str = &author_surname.to_uppercase();

    let mut author_name: String = String::new();
    for name in &author_parts {
        let mut name_str: String = name.to_string(); 
        let last_part: &str = match author_parts.last() {
            Some(x) => x,
            None => panic!("Couldn't find last part of author name."),
        };
        if &last_part == name {
            author_name.push_str(name);
        } 
        else {
            name_str.push_str(" ");
            author_name.push_str(&name_str);
        }
    }

    let mut result = String::new();
    let author_str: String = author_surname.to_string() + ", " + &author_name;
    result.push_str(&author_str);

    return result;
}
