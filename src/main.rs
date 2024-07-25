use std::fs;

fn main() {
    let page = fs::read_to_string("./template.html").expect("Deveria ter lido o arquivo...");
    let page = page.replace("{}", "Hello World!");
    fs::write("./output/index.html", page).expect("Não consegui escrever no arquivo ;-;");
}
