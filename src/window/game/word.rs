#[derive(Debug, Clone)]
pub struct Word {
    to_guess: String,
    pub current: String,
    fails: u8,
}

impl Word {
    pub fn new() -> Self {

        Word {
            to_guess: "".to_string(),
            current: "".to_string(),
            fails: 0
        }
    }

    pub fn next_word(&mut self, word: String) {
        // create placeholder word
        let current = "_".repeat(word.len());

        println!("current word: {}", current);

        self.to_guess = word;
        self.current = current;
        self.fails = 0;
    }

    pub fn check(&mut self, guess: char) -> (bool, u8) {
        let mut count = 0;

        println!("current guess: \n\tto_guess: {},\n\tcurrent: {},\n\tguess: {}", self.to_guess, self.current, guess);

        // "key/value" loop over guessable word
        for (p, c) in self.to_guess.chars().enumerate() {
            // and check if char to guess exists at index
            if c == guess {
                // replace placeholder _ char at pos with real char (as &str)
                self.current.replace_range(p..p+1, &guess.to_string()[0..]);
                // if char was found, increase count of found chars
                // rust doesn't work with x++!!!
                count += 1;
            }
        }

        // no guessed char was found, so we have a fail here
        if count == 0 {
            self.fails += 1;
        }

        // return tuple with whether solveable and guessed word are equal and how many chars were found
        (self.to_guess == self.current, count)
    }

    pub fn draw(&self) {
        // draw current guessed word
        println!("{}", self.current);
    }

    pub fn get_fails(&self) -> u8 {
        // return current fails
        self.fails
    }

    pub fn solve(&self) -> String {
        // return guessable word
        self.to_guess.clone()
    }
}

