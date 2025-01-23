use std::time::Instant;

fn main() {
    let target_pin = "905098"; // PIN koden programmet skal finne
    let pin_length = target_pin.len(); // finner lengden av PIN koden over

    println!("Starter et brute force angrep på en PIN som er {} tall lang | PIN: {}", pin_length, target_pin);
    println!("-----------------------------------------------------------------------");

    let start_time = Instant::now(); // starter en timer
    let max_attempts = 10_u32.pow(pin_length as u32); // lager en variabel med verdian av 10 (som en u32) opphøyd i pin_length (som u32)

    for i in 0..max_attempts {
        let current_attempt = format!("{:0width$}", i, width = pin_length); // formater tallet med den riktige mengden 0-er basert på PIN-lengden (er PIN-koden 4 tall lang, er det 4 nuller, osv)

        println!("Prøver: {}", current_attempt); // printer ut hvert forsøk

        if current_attempt == target_pin {
            let duration = start_time.elapsed();
            println!("\n=====================");
            println!("PIN found! The code is: {}", current_attempt);
            println!("Attempts needed: {}", i + 1);
            println!("Time taken: {:.2?}", duration);
            println!("=====================");
            break;
        }
    }
}
