# Boleto Utils CLI

This package exposes functionalities provided by the create [boleto-utils] as a CLI.

> As the subject of this project is specific to Brazil, this document is written in brazilian portuguese.

## Instalação

```sh
$ cargo install boleto-utils-cli
```

Garanta que `~/.cargo/bin` está no seu PATH para que binários instalados com `cargo install`
possam ser chamados diretamente no seu terminal.

## Uso

### Ajuda

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

### Informações

Para ajuda em como usar o subcomando `info` basta usar o comando `boleto info --help`

```sh
$ boleto info --help
Analisa o código de barra retornando os dados extraídos

USAGE:
    boleto info [OPTIONS] <COD_BARRAS>

ARGS:
    <COD_BARRAS>    Código de barras ou linha digitável

OPTIONS:
    -f, --format <FORMAT>    Formato da saída [default: text] [possible values: text, json, yaml]
    -h, --help               Print help information
    -V, --version            Print version information

```

Passando um código de barras:

```sh
$ boleto info 30195917700001452780000000002310237287225104

            Tipo: Cobrança
Código de barras: 30195917700001452780000000002310237287225104
 Linha digitável: 30190000030000231023372872251045591770000145278
           Banco: [301] BPP Instituição de Pagamento S.A.
           Moeda: Real
           Valor: 1452.78
 Data Vencimento: 2022-11-22
```

Passando uma linha digitável:

```sh
$ boleto info 868900000015238626752850720221223001810200058809

            Tipo: Arrecadação
Código de barras: 86890000001238626752857202212230081020005880
 Linha digitável: 868900000015238626752850720221223001810200058809
        Segmento: Carnês
           Valor: 123.86
```

Retornando como JSON (usando `--format json` ou `-f json`):

```sh
$ boleto info 30195917700001452780000000002310237287225104 --format json

            Tipo: Cobrança
Código de barras: 30195917700001452780000000002310237287225104
 Linha digitável: 30190000030000231023372872251045591770000145278
           Banco: [301] BPP Instituição de Pagamento S.A.
           Moeda: Real
           Valor: 1452.78
 Data Vencimento: 2022-11-22
```

Retornando como YAML (usando `--format yaml` ou `-f yaml`):

```sh
$ boleto info 30195917700001452780000000002310237287225104 --format yaml

tipo: cobranca
dados:
  cod_barras: '30195917700001452780000000002310237287225104'
  linha_digitavel: '30190000030000231023372872251045591770000145278'
  info_banco:
    id: 301
    nome: BPP Instituição de Pagamento S.A.
  cod_moeda: Real
  data_vencimento: 2022-11-22
  valor: 1452.78
```

### Dígitos verificadores

O subcomando `digito-verificador` ou `dv` recebe um código de barras ou linha digitável
possivelmente inválidos e retorna o código de barras, a linha digitável, o dígito verificador
geral e os dígitos verificadores dos campos da linha digitável calculados.

_**Obs.**: o mínimo de campos é validado para que seja possível calcular os dígitos verificadores,
ou seja, o código de barras e linha digitável retornados tem seus dígitos verificadores válidos
porém não é garantido que outros campos sejam válidos. Se você quer validar todos os campos
de um input, você deve usar o comando `info`._

Para ver ajuda do comando:

```sh
$ boleto dv --help

Calcula o dígito verificador de um código de barras validando apenas o mínimo de dados necessário
para realizar o cálculo

USAGE:
    boleto digito-verificador [OPTIONS] <COD_BARRAS>

ARGS:
    <COD_BARRAS>    Código de barras ou linha digitável

OPTIONS:
    -f, --format <FORMAT>    Formato da saída [default: text] [possible values: text, json, yaml]
    -h, --help               Print help information
    -V, --version            Print version information
```

Calculando valores de código de barras:

```sh
$ boleto dv 12345678901234567890123456789012345678901234

        DV geral: 5
       DV campos: 0 | 3 | 3
Código de barras: 12345678901234567890123456789012345678901234
 Linha digitável: 12340123405678901234356789012343567890123456789

$ boleto dv 80800000000000000000000000000000000000000000

        DV geral: 7
       DV campos: 7 | 0 | 0 | 0
Código de barras: 80870000000000000000000000000000000000000000
 Linha digitável: 808700000007000000000000000000000000000000000000
```

Calculando valores de linha digitável:

```sh
$ boleto dv 12345678901234567890123456789012345678901234567

        DV geral: 5
       DV campos: 0 | 3 | 3
Código de barras: 12345678901234567890123456789012345678901234
 Linha digitável: 12340123405678901234356789012343567890123456789

$ boleto dv 808000000000000000000000000000000000000000001111

        DV geral: 9
       DV campos: 7 | 0 | 0 | 2
Código de barras: 80890000000000000000000000000000000000000111
 Linha digitável: 808900000007000000000000000000000000000000001112
```

[boleto-utils]: https://crates.io/crates/boleto-utils