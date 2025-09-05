use chrono::Local;
use rand::prelude::*;
use simple_stopwatch::Stopwatch;
use std::{
    fs::{File, OpenOptions},
    io::{self, prelude::*, BufReader},
    path::Path,
    time::Duration, usize,
};
use whoami::username;

// funksjon for å lese linjer fra fil
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file :(");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line :("))
        .collect()
}

fn main() {
    // henter brukernavn og sier hei til brukeren
    let username = username();
    println!("Heisann, {username}");

    // spør hvor mange ord brukeren vil ha i skrivetesten
    println!("Hvor mange ord vil du skrive i skrivetesten?");
    let mut amount_of_words_wanted_input = String::new();
    io::stdin()
        .read_line(&mut amount_of_words_wanted_input)
        .expect("Failed to read line for amount_of_words_wanted_input");

    // trimmer og parser amount_of_words_wanted og gjør det om til en usize
    let amount_of_words_wanted = amount_of_words_wanted_input
        .trim()
        .parse::<usize>()
        .unwrap_or(10);

    // henter ord fra fil norwegian.txt
    let words = lines_from_file("src/norwegian-dictionary.txt");
    println!("\nOrdboken har {:?} ord", words.len());

    // velger brukerspesifisert antall tilfeldige ord fra ordlisten (norwegian.txt)
    let mut rng = rand::rng();
    let selection: Vec<_> = words
        .choose_multiple(&mut rng, amount_of_words_wanted)
        .cloned()
        .collect();

    // åpner resultat filen. hvis den ikke finnes, så lager den den.
    let mut results_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("results/speeds.txt")
        .expect("Can't find speeds.txt :(");

    // spør om brukeren er klar for å skrive
    println!("Er du klar for å skrive? (Trykk enter)");
    let mut ready = String::new();
    io::stdin()
        .read_line(&mut ready)
        .expect("Failed to read line to see if the user is ready");
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

    let text = selection.join(" ");
    let amount_of_characters: Vec<_> = text.split("").collect();
    println!("Yo, du skal skrive dette:\n{text}");
    let stopwatch = Stopwatch::start_new();

    // user input for den faktiske skrive delen
    let mut user_text = String::new();
    io::stdin()
        .read_line(&mut user_text)
        .expect("Failed to read user input for main part of test");

    // henter hvor mye tid brukeren brukte på å skrive
    let user_time = stopwatch.s();
    let amount_of_words = amount_of_characters.len() / 5;
    let words_per_minute = amount_of_words as f32 / user_time as f32 * 60.0;
    let local_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // hvis burkeren skrev riktig :)
    if user_text.trim_end() == text {
        println!("Du brukte {user_time} sekunder på å skrive det.");
        println!("Du skrev {:?} bokstaver.", amount_of_characters.len());
        println!("Ord i minuttet: {words_per_minute}");
        write!(
            results_file,
            "Brukernavn: {:?} \n
            Tid: {:?}\n
            Resultat: {:?} WPM\n
            Ord skrevet: {:?}\n \n",
            username, local_time, words_per_minute, text
        )
        .expect("Kunne ikke skrive :(")
    }
    // hvis brukeren skrev feil :(
    else {
        println!("Det du skrev matchet ikke det du skulle skrive. Bad boy.");
        write!(results_file, "Brukernavn: {:?} \nTid: {:?}\nResultat: SKRIVETEST FEILET \nOrd skrevet: {:?} \nSkrivetest:  {:?} \n \n", username, local_time, user_text.trim_end(), text).expect("Kunne ikke skrive :(");
    }
}
