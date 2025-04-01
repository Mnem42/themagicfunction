use rand::Rng;

pub struct CodeRunner{
    strings: Vec<String>
}

impl CodeRunner{
    pub fn new(strings: &[String]) -> Self{
        CodeRunner{
            strings:strings.to_vec()
        }
    }

    pub fn run_code_cli(self){
        let mut rng = rand::rng();
        println!("{}",self.strings[rng.random_range(0..self.strings.len())]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /*#[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }*/
}
