use anyhow::{Result, anyhow};
use boleto_utils::arrecadacao::{CodBarras as CodBarrasArr, LinhaDigitavel as LinhaDigitavelArr};
use boleto_utils::cobranca::{CodBarras as CodBarrasCob, LinhaDigitavel as LinhaDigitavelCob};
use boleto_utils::{Boleto, BoletoError};
use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[clap(
    version,
    about,
    propagate_version = true,
    arg_required_else_help = true
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Analisa o código de barra retornando os dados extraídos.
    #[clap(arg_required_else_help = true, visible_alias = "i")]
    Info(BarcodeInput),
    /// Calcula o dígito verificador de um código de barras validando
    /// apenas o mínimo de dados necessário para realizar o cálculo.
    #[clap(arg_required_else_help = true, visible_alias = "dv")]
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

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        None => return Err(anyhow!("Comando não encontrado, use --help para ajuda.")),
        Some(Commands::Info(input)) => {
            let boleto = Boleto::new(input.cod_barras.as_bytes())?;

            match input.format {
                Format::Text => println!("{}", boleto),
                Format::Json => println!("{}", serde_json::to_string_pretty(&boleto)?),
                Format::Yaml => println!("{}", serde_yaml::to_string(&boleto)?),
            }
        }
        Some(Commands::DigitoVerificador(input)) => {
            let input = input.cod_barras.as_bytes();

            match input.first() {
                None => return Err(anyhow::Error::new(BoletoError::InvalidLength)),
                Some(b'8') => {
                    let (cod_barras, linha_digitavel) = match input.len() {
                        44 => {
                            let cod_barras = CodBarrasArr::new(input)?;
                            let linha_digitavel: LinhaDigitavelArr = (&cod_barras).try_into()?;
                            (cod_barras, linha_digitavel)
                        },
                        48 => {
                            let linha_digitavel = LinhaDigitavelArr::new(input)?;
                            let cod_barras: CodBarrasArr = (&linha_digitavel).into();
                            (cod_barras, linha_digitavel)
                        },
                        _ => return Err(anyhow::Error::new(BoletoError::InvalidLength)),
                    };
                    let dv = cod_barras.calculate_dv();
                    let dvs = cod_barras.calculate_dv_campos();

                    // atualizando dv dos campos
                    let mut cod_barras = *cod_barras;
                    cod_barras[3] = dv + b'0';

                    let mut linha_digitavel = *linha_digitavel;
                    linha_digitavel[11] = dvs.0;
                    linha_digitavel[23] = dvs.1;
                    linha_digitavel[35] = dvs.2;
                    linha_digitavel[47] = dvs.3;
                    linha_digitavel[3] = dv + b'0';

                    println!(
                        concat!(
                            "        DV geral: {}\n",
                            "       DV campos: {} | {} | {} | {}\n",
                            "Código de barras: {}\n",
                            " Linha digitável: {}",
                        ),
                        dv,
                        dvs.0 - b'0',
                        dvs.1 - b'0',
                        dvs.2 - b'0',
                        dvs.3 - b'0',
                        unsafe { std::str::from_utf8_unchecked(cod_barras.as_ref()) },
                        unsafe { std::str::from_utf8_unchecked(linha_digitavel.as_ref()) },
                    );
                },
                _ => {
                    let (cod_barras, linha_digitavel) = match input.len() {
                        44 => {
                            let cod_barras = CodBarrasCob::new(input)?;
                            let linha_digitavel: LinhaDigitavelCob = (&cod_barras).try_into()?;
                            (cod_barras, linha_digitavel)
                        },
                        47 => {
                            let linha_digitavel = LinhaDigitavelCob::new(input)?;
                            let cod_barras: CodBarrasCob = (&linha_digitavel).into();
                            (cod_barras, linha_digitavel)
                        },
                        _ => return Err(anyhow::Error::new(BoletoError::InvalidLength)),
                    };

                    let dv = cod_barras.calculate_dv();
                    let dvs = cod_barras.calculate_dv_campos();

                    // atualizando dv dos campos
                    let mut cod_barras = *cod_barras;
                    cod_barras[4] = dv + b'0';

                    let mut linha_digitavel = *linha_digitavel;
                    linha_digitavel[9] = dvs.0;
                    linha_digitavel[20] = dvs.1;
                    linha_digitavel[31] = dvs.2;
                    linha_digitavel[32] = dv + b'0';

                    println!(
                        concat!(
                            "        DV geral: {}\n",
                            "       DV campos: {} | {} | {}\n",
                            "Código de barras: {}\n",
                            " Linha digitável: {}\n",
                        ),
                        dv,
                        dvs.0 - b'0',
                        dvs.1 - b'0',
                        dvs.2 - b'0',
                        unsafe { std::str::from_utf8_unchecked(cod_barras.as_ref()) },
                        unsafe { std::str::from_utf8_unchecked(linha_digitavel.as_ref()) },
                    );
                },
            }
        },
    }

    Ok(())
}
