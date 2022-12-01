# Boleto Utils

This library aims to parse and extract information from barcodes and digitable lines that follows FEBRABAN and BACEN layouts for "boletos" (brazilian payment slips).

This document will be written in brazilian portuguese because I believe anyone who will actually use it, will be probably brazilian. If I am wrong and you do wish a README in english, please open an issue on GitHub. :)


## Roadmap

- [X] Create a CLI;
- [ ] Create a NodeJS binding library;
- [ ] Create a Python binding library;
- [ ] Create WASM optimised version aimed to run on Browser;
- [ ] Create a GitHub Page which exposes the features of the WASM version;

## CLI usage

 ```sh
 $ boleto --help
USAGE:
    boleto [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    digito-verificador    Calcula o dígito verificador de um código de barras validando apenas o
                              mínimo de dados necessário para realizar o cálculo [aliases: dv]
    help                  Print this message or the help of the given subcommand(s)
    info                  Analisa o código de barra retornando os dados extraídos [aliases: i]
 ```