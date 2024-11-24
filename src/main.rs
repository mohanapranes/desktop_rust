use services::json_parser;
mod services;

fn main() {
    let json_data = r#"
    {
        "service": "Git",
        "action": "Commit",
        "folder": "",
        "host": "",
        "branch": "",
        "commit": "",
        "files": ""
    }
    "#;

    if let Err(error) = json_parser::parse_json(json_data) {
        eprintln!("Error processing request: {}", error);
        std::process::exit(1);
    }
}
