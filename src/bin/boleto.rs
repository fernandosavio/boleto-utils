use boleto_utils::Boleto;
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[clap(
    version,
    about,
    propagate_version = true,
    arg_required_else_help = true,
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Analisa o código de barra retornando os dados extraídos.
    #[clap(
        arg_required_else_help = true,
        visible_alias = "i",
    )]
    Info(BarcodeInput),
    /// Calcula o dígito verificador de um código de barras validando
    /// apenas o mínimo de dados necessário para realizar o cálculo.
    #[clap(
        arg_required_else_help = true,
        visible_alias = "dv",
    )]
    DigitoVerificador(BarcodeInput),
}

#[derive(Args)]
struct BarcodeInput {
    /// Código de barras ou linha digitável
    #[clap(value_parser)]
    cod_barras: String,

    /// Formato da saída
    #[clap(arg_enum, short, long, value_parser, default_value_t=Format::Text)]
    format: Format,
}

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Text,
    Json,
    Yaml,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Info(input)) => {
            let boleto = Boleto::new(input.cod_barras.as_bytes()).unwrap();

            match input.format {
                Format::Text => println!("{boleto}"),
                Format::Json => println!("{}", serde_json::to_string_pretty(&boleto).unwrap()),
                Format::Yaml => println!("{}", serde_yaml::to_string(&boleto).unwrap()),
            }
        }
        Some(Commands::DigitoVerificador(input )) => {
            let boleto = Boleto::new(input.cod_barras.as_bytes()).unwrap();
            println!("digito-verificador - cod_barras: {:?}", boleto);
        },
        None => println!("Comando não encontrado, use --help para ajuda."),
    }
}
