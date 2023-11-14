use clap::{Parser, Subcommand};
use one_two_pay_api::{Bank, Client, QueryReq, TransferReq};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, env = "API_KEY", hide_env_values = true)]
    api_key: String,

    #[arg(short, long, env = "PARTNER_CODE")]
    partner_code: String,

    #[arg(short, long, env = "CHANNEL")]
    channel: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Make transfer to bank account
    Transfer {
        /// Bank account number
        #[arg(long)]
        bankacc: String,
        /// Which bank to transfer to
        #[arg(long, value_enum)]
        bank: Bank,
        /// Amount of money to transfer
        #[arg(long)]
        amount: f64,
        /// Account name, owner of the bank account
        #[arg(long)]
        accname: String,
        /// Mobile number in Thai local format
        #[arg(long)]
        mobileno: String,
        /// Reference by which system was made the transaction
        #[arg(long)]
        transaction_by: String,
        /// External ID of transaction. Must have length => 1 and <= 30.
        #[arg(long)]
        ref1: String,
        /// Additional data
        #[arg(long)]
        ref2: Option<String>,
        /// Additional data
        #[arg(long)]
        ref3: Option<String>,
        /// Additional data
        #[arg(long)]
        ref4: Option<String>,
        /// Meaning is unknown
        #[arg(long)]
        line_token: Option<String>,
        /// Meaning is unknown alas that is email address
        #[arg(long)]
        email: Option<String>,
    },
    /// Querying status of payment
    Inquery {
        /// ID of transaction
        #[arg(short, long)]
        ref1: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();
    let client = Client::new(&cli.channel, &cli.partner_code, &cli.api_key);
    match cli.command {
        Commands::Transfer {
            bankacc,
            bank,
            amount,
            accname,
            mobileno,
            transaction_by,
            ref1,
            ref2,
            ref3,
            ref4,
            line_token,
            email,
        } => {
            let res = client
                .transfer(TransferReq {
                    bankacc,
                    bank,
                    amount,
                    accname,
                    mobileno,
                    transaction_by,
                    ref1,
                    ref2,
                    ref3,
                    ref4,
                    line_token,
                    email,
                })
                .await?;
            println!("{:?}", res);
        }
        Commands::Inquery { ref1 } => {
            let res = client.query(QueryReq { ref1 }).await?;
            println!("{:?}", res);
        }
    }
    Ok(())
}
