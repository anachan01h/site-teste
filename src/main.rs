use std::fs;

fn main() {
    let page = fs::read_to_string("./template.html").expect("Deveria ter lido o arquivo...");
    let page = page.replace("{}", "Hello World!");
    fs::create_dir_all("./_site").expect("Não consegui criar a pasta ;-;");
    fs::write("./_site/index.html", page).expect("Não consegui escrever no arquivo ;-;");
}
