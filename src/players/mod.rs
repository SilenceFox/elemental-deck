// HACK: permite vencer ou perder, porém falta print.
/// API: `let score = Score::new();` `score.increment();` `score.decrement();`
/// Checa por vitorias sozinho, atualmente crasha o programa se chegar a 10 vitorias.
mod logic {
    #[derive(Debug)]
    /// HACK: 2.4 Player pode competir com a máquina
    pub struct Score {
        points: u8,
        wins: u8,
        losses: u8,
        won: bool,
    }
    impl Score {
        pub fn new() -> Self {
            Score {
                points: u8::default(),
                wins: u8::default(),
                losses: u8::default(),
                won: false,
            }
        }
        pub fn increment(&mut self) {
            self.points += 1;
            self.wins += 1;
            self.check_won();
        }
        pub fn decrement(&mut self) {
            self.points -= 1;
            self.losses += 1;
            self.check_won()
        }
        fn won(&mut self) -> Option<Self> {
            match self.wins {
                0..=9 => None,
                10 => Some(Self {
                    points: self.points,
                    wins: self.wins,
                    losses: self.losses,
                    won: true,
                }),
                _ => unreachable!("Should not be possible"),
            }
        }
        fn check_won(&mut self) {
            match self.won() {
                Some(_) => {
                    println!("Player won the game!");
                    panic!("Stop the game!");
                }
                None => (),
            }
        }
    }
}
mod computer {} //TODO:

pub mod human {
    // use super::logic::Score;

    // TODO: 2. Criar player e máquina
    // FIX: 2.1 Player pode ter um deck
    // FIX: 2.2 Player pode embaralhar seu deck
    // FIX: 2.3 Player pode descartar cartas e comprar novas
    //
    // struct Player {
    //     points: Score,
    //     // deck: Deck  FIX: Criar o deck!
    // }
    // impl Player {
    //     pub fn new() -> Self {
    //         //TODO: Build with deck of cards
    //     }
    // }
}
pub use logic::*;
// pub use human::*;
