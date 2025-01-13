use rand::Rng; // bruk Rng (random number  generator) fra rand
use std::cmp::Ordering; // bruk cmp (compare)::Ordering fra std (standard)
use std::io; // bruk io (input/output) fra std (standard)

fn main() {
    println!("Gjett tallet! \n \n");
    let secret_number = rand::thread_rng().gen_range(1..=100); // genererer et tilfeldig tall mellom 1 og 100 // det funker slik: gen_range(start..=end)
    loop {
        // loop for at brukeren skal kunne gjette flere ganger
        println!("Vennligst skriv inn ditt gjett.");

        let mut guess = String::new(); // lager en mutable variabel som heter guess, og assigner den til en ny string (basically en midlertidig tom string), som blir fylt in av linjen under med user input

        io::stdin() // bruker input funksjonen fra std::io
            .read_line(&mut guess) // read line leser linjen (stringen) som brukeren skrev inn, og sender den til variabelen 'guess' | '&' betyr at det er en referanse, som i denne tilfellen refererer til variabelen 'guess'
            .expect("Kunne ikke lese linjen."); // hvis 'result' (funksjonen .expect bruker) er 'Err', så skriv "Kunne ikke lese linjen" + error

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Skriv inn noe ordentlig da, din ape guling");
                continue;
            },
        }; // trimmer all whitespace i user input stringen 'guess', og gjør den om til en u32 integer.

        match guess.cmp(&secret_number) {
            // bruker compare på guess og secret number
            Ordering::Less => println!("For lite! \n"), // hvis guess er mindre enn secret number
            Ordering::Greater => println!("For stort! \n"), // hvis guess er større enn secret number
            Ordering::Equal => {
                println!(
                    "Du klarte det ez 4 ence ence ence ence!! Tallet var: {}",
                    secret_number
                ); // hvis guess er lik secret number
                break; // exiter loopen og dreper programmet fordi hele programmet er inni loopen
            }
        } // match compare
    } // loop
} // fn main
