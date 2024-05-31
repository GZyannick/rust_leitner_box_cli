use std::{io, time};

pub mod cards {
    #[derive(Debug)]
    pub struct Card {
        pub id: usize,
        pub question: String,
        pub reponse: String,
        pub current_box: u8, // there are 5 box each box is an integer that correspond to the
                             // number of day before the question comeback to the user
                             // after box 5 the concept is learned
                             /* date: not implemented yet */
    }

    impl Card {
        pub fn next_box(&mut self) {
            if self.current_box < 5 {
                self.current_box += 1;
            }
        }

        pub fn prev_box(&mut self) {
            if self.current_box > 0 {
                self.current_box -= 1;
            }
        }
    }
}

use crate::leitner::cards::Card;
#[derive(Debug)]
pub struct Store {
    pub vec: Vec<Card>,
}

fn ask_for(ask: String) -> String {
    let mut val = String::new();
    println!("{}", ask);
    io::stdin().read_line(&mut val).expect("Échec de l'entrée");
    val
}

impl Store {
    pub fn new() -> Store {
        Store { vec: Vec::new() }
    }

    pub fn add(&mut self) {
        let question = ask_for(String::from("Entrer votre question"));
        let reponse = ask_for(String::from("Entrer vortre réponse"));

        self.vec.push(Card {
            id: self.vec.len() + 1,
            question,
            reponse,
            current_box: 0,
        })
    }

    pub fn show(&self) {
        if self.vec.len() == 0 {
            println!("--- Aucune Question ---");
            return;
        }
        for card in &self.vec {
            println!();
            println!("-----------------------------------------------------------------------------------");
            println!("id          : {}", card.id);
            print!("question    : {}", card.question);
            print!("réponse     : {}", card.reponse);
            println!("current_box : {}", card.current_box);
            println!("-----------------------------------------------------------------------------------");
        }
    }

    pub fn delete(&mut self) {
        self.show();

        println!("choose which one you want to delete with the id)");
        println!("press Enter without value or with a false id");
        let mut index = String::new();
        io::stdin()
            .read_line(&mut index)
            .expect("Échec de l'entrée");
        let index = match index.trim().parse::<usize>() {
            Ok(nombre) => nombre - 1,
            Err(_) => return,
        };

        if index <= self.vec.len() {
            self.vec.remove(index);
        }
    }

    pub fn daily_question(&mut self) {
        if self.vec.len() == 0 {
            println!("--- Aucune Question ---");
            return;
        }

        fn handle_question_choice(card: &mut Card) {
            let mut res = String::new();
            io::stdin().read_line(&mut res).expect("Échec de l'entrée");
            let res: u32 = match res.trim().parse() {
                Ok(nombre) => nombre,
                Err(_) => return, // a voir si je ne fait pas un appel a soit meme
            };
            match res {
                1 => {
                    print!("réponse: {}", card.reponse);
                    handle_question_choice(card);
                }
                2 => card.next_box(),
                3 => card.prev_box(),
                _ => handle_question_choice(card),
            }
        }

        println!("------------------------------------");
        for card in &mut self.vec {
            println!();
            println!("id: {}, question: {}", card.id, card.question);
            println!("1. Afficher la réponse, 2. Vrai, 3. Faux");
            handle_question_choice(card);
            println!();
        }
        println!("------------------------------------");
    }
}

pub mod app {
    fn print_choice() {
        println!();
        println!("1. Ajouter une question");
        println!("2. Répondre aux questions");
        println!("3. Afficher toutes les questions");
        println!("4. supprimer une question");
        println!("5. stop");
        println!();
    }

    pub fn start() {
        use std::io;
        let mut leitner_box = crate::leitner::Store::new();
        loop {
            let mut buffer = String::new();
            print_choice();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Échec de l'a lecture");

            let buffer: usize = match buffer.trim().parse() {
                Ok(nombre) => nombre,
                Err(_) => continue,
            };

            match buffer {
                1 => leitner_box.add(),
                2 => leitner_box.daily_question(),
                3 => leitner_box.show(),
                4 => leitner_box.delete(),
                5 => break,
                _ => continue,
            }
        }
    }
}
