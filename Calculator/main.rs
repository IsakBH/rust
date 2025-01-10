use std::io; // bruk io (input/output) fra std (standard)

fn main() {
    loop {
        /////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
        println!("Tøff kalkualtor?");

        //////////////////////////////////////////////////////////// DET FØRSTE TALLET | TALL 1 /////////////////////////////////////////////////////////////////////////////
        println!("Skriv inn tall 1 :)"); // spør brukeren om å skrive inn tall 1
        let mut calculator1 = String::new();

        io::stdin()
            .read_line(&mut calculator1)
            .expect("Skriv noe ordentlig da din apekatt.");

        let calculator1: u32 = calculator1.trim().parse().expect("Vennligst skriv inn et tall!");

        //////////////////////////////////////////////////////////// DET ANDRE TALLET | TALL 2 /////////////////////////////////////////////////////////////////////////////
        println!("Skriv inn tall 2 :)"); // spør brukeren om å skrive inn tall 2
        let mut calculator2 = String::new();

        io::stdin()
            .read_line(&mut calculator2)
            .expect("Skriv noe ordentlig da din apekatt.");

        let calculator2: u32 = calculator2.trim().parse().expect("Vennligst skriv inn et tall!");

        //////////////////////////////////////////////////////////// OPERATOREN  /////////////////////////////////////////////////////////////////////////////
        println!("Skriv inn operator :)"); // spør brukeren om å skrive inn operatoren programmet skal bruke
        let mut operator = String::new();

        io::stdin()
            .read_line(&mut operator)
            .expect("Bro??? Er det så vanskelig å skrive noe ordentlig?");

        let operator: &str = operator.trim();

        if operator == "+" {
            println!("Resultat: {} \n", calculator1 + calculator2);
        } else if operator == "-" {
            println!("Resultat: {} \n", calculator1 - calculator2);
        } else if operator == "*" {
            println!("Resultat: {} \n", calculator1 * calculator2);
        } else if operator == "/" {
            println!("Resultat: {} \n", calculator1 / calculator2);
        } else {
            println!("Du skrev noe feil. \n");
            println!("Vennligst skriv inn et tall. \n");
            println!("Vennligst skriv inn et tall. \n");
        }

    }
}
