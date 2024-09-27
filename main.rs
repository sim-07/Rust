use rand::Rng;
use std::io;

fn main() {
    let n_casuale = rand::thread_rng().gen_range(1..=100);
    
    println!("Numero: ");

    loop {
        let mut n_inserito = String::new();

        io::stdin()
            .read_line(&mut n_inserito)
            .expect("Errore");


        let n_inserito: u32 = match n_inserito.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                print!("Numero non valido");
                continue;
            }
        };

        
        if n_inserito > n_casuale {
            println!("Troppo alto");
        } else if n_inserito < n_casuale {
            println!("Troppo basso");
        } else if n_inserito == n_casuale {
            println!("Giusto");
            break;
        }
        
    }
}
