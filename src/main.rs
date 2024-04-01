#![warn(clippy::complexity, clippy::style, clippy::perf)]
use color_eyre::eyre::{Report, Result};


mod macros;
mod players;
use players::*;
// TODO: # Jogo de cartas elementais
// TODO: 1. Criar deck
// TODO: 1.1 Criar cartas
// TODO: 1.2 Embaralhar cartas
// TODO: 1.3 Descartar e comprar cartas
// TODO: 1.4 Cartas com elementos
//
//
// TODO: Do a thing.
// FIX: Literally broken.
// HACK: Its working, but God forbid a soul of laying eyes upon it.
// DONE: Somehow you did it.
// PERF: Your code is unperformant, nice.
// NOTE: A little note, how cool.
// TEST: Im not even sure this runs, so please test it.

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    // Actual code
    println!("Hello, world!");
    let mut player = Score::new();
    for _ in 0..20 {
        player.increment();
        player.increment();
        println!("inc{:#?}", player);
        player.decrement();
        println!("dec{:#?}", player);
    }
    // End of Actual code
    Ok(())
}
