#[derive(Debug)]
struct GameState {
    marbles: Vec<u64>,
    current_marble: usize,
    current_marble_value: u64,
    players: Vec<u64>,
    current_player: usize,
}

impl GameState {
    fn new(players: usize) -> Self {
        let mut marbles = Vec::with_capacity(7130700);
        marbles.push(0);
        GameState {
            marbles,
            current_marble: 0,
            current_marble_value: 1,
            players: vec![0; players],
            current_player: 0,
        }
    }

    pub fn step(&mut self) {
        if self.current_marble_value % 23 == 0 {
            self.add_scores();
        } else {
            self.insert_marble();
        };

        self.current_player = (self.current_player + 1) % self.players.len();
        self.current_marble_value += 1;
    }

    fn insert_marble(&mut self) {
        let insert_location = ((self.current_marble + 1) % self.marbles.len()) + 1;
        self.marbles
            .insert(insert_location, self.current_marble_value);
        self.current_marble = insert_location;
    }

    fn add_scores(&mut self) {
        let mut scores_to_add = self.current_marble_value as u64;

        let marble_to_remove = (self.current_marble + self.marbles.len() - 7) % self.marbles.len();
        let removed_marble = self.marbles.remove(marble_to_remove);

        scores_to_add += removed_marble;
        self.players[self.current_player] += scores_to_add;

        self.current_marble = marble_to_remove % self.marbles.len();
    }
}

fn main() {
    run_game(439, 7130700);
}

fn run_game(players: usize, max_steps: u64) -> GameState {
    // Solution on linked list would to be much faster
    let mut state = GameState::new(players);

    for i in 0..max_steps {
        if i % 100 == 0 {
            print!(
                "{}/{} ({:.3}%)\r",
                i,
                max_steps,
                i as f64 / max_steps as f64 * 100.0
            );
        }
        state.step();
    }
    print!(
        "{}/{} ({:.3}%)\r",
        max_steps,
        max_steps,
        max_steps as f64 / max_steps as f64 * 100.0
    );
    println!();

    println!("Marbles: {:?}", state.marbles);
    println!("Marbles len: {:?}", state.marbles.len());
    println!("Current: {:?}", state.current_marble);
    println!("Player: {:?}", state.current_player);
    println!("Players: {:?}", state.players);
    println!();
    println!("Max score: {}", state.players.iter().max().unwrap());

    state
}
