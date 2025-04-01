pub struct CodeRunner{
    strings: Vec<String>
}

impl CodeRunner{
    fn new(strings: Vec<String>) -> Self{
        CodeRunner{
            strings
        }
    }

    fn run_code_cli(){
        println!("Test");
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
