use gtk::prelude::*;
use rand::Rng;

mod word;
use word::*;

#[derive(Debug, Clone)]
pub struct Game {
    words: Vec<String>,
    points: u32,
    fails: u8,
    max_fails: u8,
    pub word: Word,
    builder: Box<gtk::Builder>,
    pub running: bool
}

impl Game {
    pub fn new(builder: gtk::Builder, max_fails: u8) -> Self {
        // get words
        // TODO: get them from elsewhere
        let pool_of_words = vec![
            "rust",
            "programing",
            "language",
            "learning",
            "doing",
            "hangman",
            "game",
            "structs",
            "loops",
            "stdin",
            "string",
            "conversions",
            "linux",
            "gnome",
            "desktop",
            "gtk",
            "opensource",
            "actually",
            "fun",
        ];

        let mut words: Vec<String> = Vec::new();

        for word in pool_of_words {
            words.push(word.to_string());
        }

        Game {
            words,
            points: 0,
            fails: 0,
            max_fails: max_fails,
            word: Word::new(),
            builder: Box::new(builder),
            running: false
        }
    }

    pub fn start(&mut self, reset: bool) {
        // int randomizer, has to be mut!
        let mut rng = rand::thread_rng();

        // get random index of pool of words
        let n: u8 = rng.gen_range(0, self.words.len() as u8);
        // get random word
        let word_to_guess = &self.words[n as usize];
        println!("word_to_guess: {}", word_to_guess);

        if reset {
            self.points = 0;
            self.fails = 0;
            self.running = true;
        }

        // init word
        self.word.next_word(word_to_guess.clone());

        self.render_points();
        self.render_board();
    }

    pub fn guess(&mut self, letter: char) -> bool {
        println!("letter given: {}", letter);

        let (solved, counts) = self.word.check(letter);

        // increase points if counts > 0
        if counts > 0 {
            self.points += 1;
        } else {
            // get current fails
            self.fails += 1;
        }

        if solved {
            // solved, show current board
            // word.draw();
            println!("you found the word!");
            return true;
        }
        else if self.max_fails > self.fails {
            // not solved but still allowed to try
            // show current board
            // word.draw();
        } else {
            // no trials left, show solved word
            println!("game over! the word was: {}", self.word.solve());
            self.running = false;
        }

        false
    }

    pub fn render_board(&mut self) {
        let board: gtk::Box = self.builder.get_object("container_word").unwrap();

        println!("destroying existing board");
        println!("in render_board: {}", self.word.current);
        for label in board.get_children() {
            label.destroy();
        }

        for (_, c) in self.word.current.chars().enumerate() {
            let label = gtk::Label::new(None);
            let mut ch = ' ';

            if c != '_' {
                ch = c;
            }

            label.set_text(&ch.to_string()[0..]);
            // label.set_text("_");
            label.show();
            board.add(&label);
        }
    }

    pub fn render_points(&mut self) {
        println!("points: {}", self.points);
        println!("fails: {}/{}", self.fails, self.max_fails);
        let points: gtk::Label = self.builder.get_object("points").unwrap();
        points.set_text(&self.points.to_string()[..]);

        let fails: gtk::Label = self.builder.get_object("fails").unwrap();
        let str_fails: String = self.fails.to_string() + "/" + &self.max_fails.to_string();
        fails.set_text(&str_fails[..]);
    }

    pub fn quit() {
        // clean up
    }
}
