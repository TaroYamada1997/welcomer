use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    about,
    long_about = "This is a simple CLI program to say hello in different languages."
)]
struct Args {
    #[arg(short = 'n', long = "n")]
    name: String,

    #[arg(short = 'l', long = "l")]
    language: String,
}

fn main() {
    let args = Args::parse();

    let name = args.name;

    match args.language.as_str() {
        "en" => println!("Hello, {name}!"),
        "ja" => println!("こんにちは、{name}！"),
        "zh" => println!("你好，{name}！"),
        "es" => println!("¡Hola, {name}!"),
        "fr" => println!("Bonjour, {name}!"),
        "de" => println!("Hallo, {name}!"),
        "it" => println!("Ciao, {name}!"),
        "ru" => println!("Привет, {name}!"),
        "ko" => println!("안녕하세요, {name}!"),
        "zu" => println!("Sawubona, {name}!"),
        _ => println!("Hello, {name}!"),
    }
}
