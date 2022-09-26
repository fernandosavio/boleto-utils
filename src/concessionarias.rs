extern crate csv;

use std::collections::HashMap;
use std::fs;

use lazy_static::lazy_static;

use crate::arrecadacao::Segmento;


lazy_static! {
    static ref PREFEITURAS: HashMap<u16, InfoConvenio> = {
        let file = fs::File::open("data/concessionarias-1-prefeituras.csv")
            .expect("File not opened");
        let mut reader = csv::Reader::from_reader(file);

        reader.records()
            .map(|line| {
                let record = line.expect("Line not parsed");
                let banco: InfoConvenio = InfoConvenio {
                    id: record[1].parse().expect("id not parsed"),
                    nome: record[0].to_string(),
                };
                (banco.id, banco)
            })
            .collect()
    };
}

#[derive(Debug)]
pub struct InfoConvenio {
    pub id: u16,
    pub nome: String,
}

impl InfoConvenio {
    pub fn get(segmento: &Segmento, id: u16) -> Option<&'static InfoConvenio> {
        match segmento {
            Segmento::Prefeituras => PREFEITURAS.get(&id),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{InfoConvenio, PREFEITURAS};
    use crate::arrecadacao::Segmento;

    #[test]
    fn should_load_and_get_prefeituras() {
        const SEG: &Segmento = &Segmento::Prefeituras;

        let info = InfoConvenio::get(SEG, 9999);
        assert!(matches!(info, None));

        println!("HashMap.len() == {}", PREFEITURAS.len());

        let info = InfoConvenio::get(SEG, 0).expect("Find by ID");
        assert_eq!(info.id, 0);
        assert!(info.nome.contains("São Paulo"));

        let info = InfoConvenio::get(SEG, 3659).expect("Find by ID");
        assert_eq!(info.id, 3659);
        assert!(info.nome.contains("Rio de Janeiro"));

        let info = InfoConvenio::get(SEG, 1319).expect("Find by ID");
        assert_eq!(info.id, 1319);
        assert!(info.nome.contains("Curitiba"));
    }
}
