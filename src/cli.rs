use clap::Parser;

/// Encrypt or decrypt a file or text from stdin
#[derive(Parser)]
#[command(author = "Szymon Błaszczyński <museyoucoulduse@gmail.com>", version, about = "Encrypt or decrypt a file or text from stdin", long_about = None)]
pub struct Cli {
    /// encrypt
    #[arg(short, long, default_value = "false")]
    pub encrypt: bool,

    /// decrypt
    #[arg(short, long, default_value = "false")]
    pub decrypt: bool,

    /// set input file
    #[arg(short, long)]
    pub input: Option<String>,

    /// set output file
    #[arg(short, long)]
    pub output: Option<String>,

    /// key from file
    #[arg(long)]
    pub key_file: Option<String>,

    /// nonce from file
    #[arg(long)]
    pub nonce_file: Option<String>,

    /// print result to stdout
    #[arg(long, default_value = "false")]
    pub stdout: bool,

}

