/* Manta Ray LLM CLI for climate care initiatives */
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Manta Ray Team")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Answer {
        #[clap(short, long)]
        question: String,
        #[clap(short, long)]
        context: String,
    },
    Calculate {
        #[clap(short, long)]
        expression: String,
    },
    Petition {
        #[clap(short, long)]
        corporation: String,
        #[clap(short, long)]
        issue: String,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Answer { question, context }) => {
            let answers = manta_ray_llm::answer_climate_question(&question, &context);
            println!("Answer: {}", answers.get(0).unwrap_or(&"No answer found".to_string()));
        }
        Some(Commands::Calculate { expression }) => {
            match manta_ray_llm::calculate(&expression) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => println!("Error: {}", e),
            }
        }
        Some(Commands::Petition { corporation, issue }) => {
            let petition = manta_ray_llm::generate_petition(&corporation, &issue);
            println!("{}", petition);
        }
        None => println!("No command given. Use --help for options."),
    }
}
