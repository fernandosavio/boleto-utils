#[cfg(test)]
mod tests {
    use boleto_utils::{Boleto, Error};

    #[test]
    fn invalid_input_error() {
        let input = "A".repeat(44);
        let result = Boleto::new(&input);

        assert!(matches!(result, Err(Error::NumbersOnly)));
    }

    #[test]
    fn invalid_length_error() {
        let invalid_lengths = [1, 10, 20, 30, 40, 43, 45, 49, 50];

        for i in invalid_lengths {
            let input = "0".repeat(i);
            let result = Boleto::new(&input);
    
            assert!(matches!(result, Err(Error::InvalidLength(l))  if l == i));
       }
    }
    
}