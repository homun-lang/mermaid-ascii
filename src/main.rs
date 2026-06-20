use clap::Parser;

#[derive(Parser)]
#[command(name = "mermaid-ascii", version = env!("MERMAID_ASCII_VERSION"))]
struct Cli {
    /// Input file (reads from stdin if omitted)
    input: Option<String>,

    /// Use plain ASCII characters instead of Unicode
    #[arg(short, long)]
    ascii: bool,

    /// Override graph direction (TD, LR)
    #[arg(short, long)]
    direction: Option<String>,

    /// Node padding
    #[arg(short, long, default_value = "1")]
    padding: usize,

    /// Write output to file instead of stdout
    #[arg(short, long)]
    output: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let text = match &cli.input {
        Some(path) => std::fs::read_to_string(path).expect("could not read input file"),
        None => {
            use std::io::Read;
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .expect("could not read stdin");
            buf
        }
    };

    let direction = cli.direction.as_deref().and_then(|d| match d {
        "TD" | "td" => Some(mermaid_ascii::Direction::TD),
        "LR" | "lr" => Some(mermaid_ascii::Direction::LR),
        _ => None,
    });

    let result = mermaid_ascii::render_dsl(&text, cli.ascii, direction);

    match &cli.output {
        Some(path) => std::fs::write(path, &result).expect("could not write output file"),
        None => print!("{}", result),
    }
}
