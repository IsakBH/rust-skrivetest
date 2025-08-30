use simple_stopwatch::Stopwatch;
use std::io;
use std::time::Duration;

fn main() {
    // spør om brukeren er klar for å skrive
    println!("Er du klar for å skrive? (Trykk enter)");
    let mut ready = String::new();
    io::stdin()
        .read_line(&mut ready)
        .expect("Failed to read line");
    println!("Brukeren er klar.");

    // teller ned fra 5 slik at brukeren kan gjøre seg klar
    println!("5..");
    std::thread::sleep(Duration::from_millis(1000));
    println!("4..");
    std::thread::sleep(Duration::from_millis(1000));
    println!("3..");
    std::thread::sleep(Duration::from_millis(1000));
    println!("2..");
    std::thread::sleep(Duration::from_millis(1000));
    println!("1..");
    std::thread::sleep(Duration::from_millis(1000));

    let text = "hei. jeg heter jonas gahr støre.";
    let amount_of_characters: Vec<_> = text.split("").collect();
    println!("Yo, du skal skrive dette:\n{text}");
    let stopwatch = Stopwatch::start_new();

    // user input for den faktiske skrive delen
    let mut user_text = String::new();
    io::stdin()
        .read_line(&mut user_text)
        .expect("Failed to read line");

    // henter hvor mye tid brukeren brukte på å skrive
    let user_time = stopwatch.s();
    let amount_of_words = amount_of_characters.len() / 5;
    let words_per_minute = amount_of_words as i32 / user_time as i32 * 60;

    // hvis burkeren skrev riktig :)
    if user_text.trim_end() == text {
        println!("Du brukte {user_time} sekunder på å skrive det.");
        println!("Du skrev {:?} bokstaver.", amount_of_characters.len());
        println!("Ord i minuttet: {words_per_minute}");
    }
    // hvis brukeren skrev feil :(
    else {
        println!("Det du skrev matchet ikke det du skulle skrive. Bad boy.");
    }
}
