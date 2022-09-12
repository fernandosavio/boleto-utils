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

#[derive(Debug)]
pub struct Arrecadacao {
    segmento: Segmento,
    tipo_valor: TipoValor,
    digito_verificador: u8,
    valor: Option<f64>,
    convenio: String,
}

impl Arrecadacao {
    pub fn new(barcode: &str) -> Result<Self, BoletoError> {
        let _ = Self::validate(barcode)?;

        const ID_BANCO: Range<usize> = 0..3;
        const COD_MOEDA: Range<usize> = 3..4;
        const DIG_VERIF: Range<usize> = 4..5;
        const FATOR_VENC: Range<usize> = 5..9;
        const VALOR: Range<usize> = 9..19;
        // const CAMPO_LIVRE: Range<usize> = 19..44;

        // let cod_banco = barcode[ID_BANCO]
        //     .parse()
        //     .expect("cod_banco deve ser numérico");

        // let cod_moeda = match barcode[COD_MOEDA]
        //     .parse()
        //     .expect("cod_banco deve ser numérico")
        // {
        //     9 => CodigoMoeda::Real,
        //     0 => CodigoMoeda::Outras,
        //     _ => return Err(BoletoError::InvalidCodigoMoeda),
        // };

        // let fator_vencimento: u16 = barcode[FATOR_VENC].parse().unwrap();

        // let valor = match barcode[VALOR].parse::<f64>().unwrap() {
        //     x if x.is_normal() => Some(x / 100.00),
        //     _ => None,
        // };

        // let digito_verificador: u8 = barcode[DIG_VERIF].parse().unwrap();

        // if digito_verificador != Self::calculate_digito_verificador(barcode)? {
        //     return Err(BoletoError::InvalidDigitoVerificador);
        // }

        Ok(Self {
            segmento: Segmento,
            tipo_valor: TipoValor,
            digito_verificador: u8,
            valor: Option<f64>,
            convenio: String,
        })
    }

    pub fn validate(barcode: &str) -> Result<(), BoletoError> {
        if barcode.len() != 44 && barcode.len() != 48 {
            return Err(BoletoError::InvalidLength);
        }

        let only_numbers = barcode.chars().all(|c| c.is_ascii_digit());
        if !only_numbers {
            return Err(BoletoError::NumbersOnly);
        }

        if barcode.bytes().next() != Some(b'8') {
            return Err(BoletoError::InvalidArrecadacaoBarcode);
        }

        Ok(())
    }
}
