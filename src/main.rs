use clap::{
    Parser,
    Subcommand,
};

/// Command-line arguments structure
#[derive(Parser, Debug)]
#[command(
    version = "0.1.0",
    about = "A tool for encoding and decoding ROT ciphers"
)]
struct Cli
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands
{
    /// Encode a string with a specified rotation shift
    #[command(short_flag = 'e')]
    Encode
    {
        /// The text to encode
        #[arg(short = 't', long)]
        text: String,

        /// The rotation shift (1-25)
        #[arg(short = 's', long)]
        shift: u8,
    },
    /// Decode a string with a specified or guessed rotation shift
    #[command(short_flag = 'd')]
    Decode
    {
        /// The text to decode
        #[arg(short = 't', long)]
        text: String,

        /// The rotation shift (1-25)
        #[arg(short = 's', long)]
        shift: Option<u8>,

        /// Guess the rotation shift automatically
        #[arg(short = 'g', long)]
        guess: bool,
    },
}

fn main()
{
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode {
            text,
            shift,
        } => {
            let encrypted = rotfor::encrypt(text, *shift);
            println!("Encoded Text: {}", encrypted);
        },
        Commands::Decode {
            text,
            shift,
            guess,
        } => {
            if *guess {
                let guess_decrypted = rotfor::auto_decrypt(text);
                println!("Auto-Decoded Text: {}", guess_decrypted);
            } else if let Some(shift) = shift {
                let decrypted = rotfor::decrypt(text, *shift);
                println!("Decoded Text: {}", decrypted);
            } else {
                eprintln!(
                    "Error: Please provide a shift value or use the --guess \
                     flag."
                );
            }
        },
    }
}
