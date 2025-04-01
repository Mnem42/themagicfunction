use std::{fs::File, io::Read, thread::sleep, time::Duration};

use progrex::ProgressBar;
use rand::Rng;

pub struct CodeRunner{
    strings: Vec<String>
}

impl CodeRunner{
    pub fn new(strings: &[String]) -> Self{
        let mut buf = String::new();
        File::open("strings.json")
            .unwrap()
            .read_to_string(&mut buf).expect("idk");

        let json = json::parse(&buf).expect("Failed to parse config");
        
        assert!(!json["strings"][0].is_null());
        assert!(json["strings"].is_array());

        CodeRunner{
            strings:json["strings"].members().map(|x| x.as_str().unwrap().to_string()).collect()
        }
    }

    pub fn run_code_cli(self){
        let mut rng = rand::rng();

        print!("\nProgress: ");
        let mut bar = ProgressBar::new(1000);

        for i in 0..=1000 {
            bar.set_progress(i);
            if(rng.random_bool(0.01)){
                sleep(Duration::from_millis(1000));
            }
        }
        println!("\n{}",self.strings[rng.random_range(0..self.strings.len())]);
    }

    pub fn run_code_gui(self){
        todo!("This is not implemented yet")
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
