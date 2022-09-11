#[derive(Debug)]
pub struct Arrecadacao {
    segmento: Segmento,
    tipo_valor: TipoValor,
    digito_verificador: u8,
    valor: Option<f64>,
    convenio: String,
}

#[derive(Debug)]
pub enum Segmento {
    Prefeituras = 1,
    Saneamento = 2,
    EnergiaEletricaEGas = 3,
    Telecomunicacoes = 4,
    OrgaosGovernamentais = 5,
    Carnes = 6,
    MultasTransito = 7,
    ExclusivoDoBanco = 9,
}


#[derive(Debug)]
pub enum TipoValor {
    ValorReaisMod10 = 6,
    QtdeMoedaMod10 = 7,
    ValorReaisMod11 = 8,
    QtdeMoedaMod11 = 9,
}

