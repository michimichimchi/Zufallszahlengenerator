/*use std::io;

fn main() {
    loop {
        println!("Zufallszahlengenerator");
    
        let mut start: String = String::new();
        let mut end: String = String::new();

        io::stdin()
            .read_line(&mut start)
            .expect("Fehler beim Lesen der Zeile");

        io::stdin()
            .read_line(&mut end)
            .expect("Fehler beim Lesen der Zeile");
        
        let start: i16 = start.trim().parse().unwrap();
        let end: i16 = end.trim().parse().unwrap();
        let random_number: i16 = rand::random_range(start..=end);

        println!("{}", random_number);
    }
}*/