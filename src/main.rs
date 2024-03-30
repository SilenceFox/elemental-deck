 #![warn(
     clippy::complexity,
     clippy::style,
     clippy::perf,
 )]
 use color_eyre::eyre::{ Result, Report };

mod macros;

// TODO: # Jogo de cartas elementais
// TODO: 1. Criar deck
// TODO: 1.1 Criar cartas
// TODO: 1.2 Embaralhar cartas
// TODO: 1.3 Descartar e comprar cartas
// TODO: 1.4 Cartas com elementos
//
// TODO: 2. Criar player e máquina
// TODO: 2.1 Player pode ter um deck
// TODO: 2.2 Player pode embaralhar seu deck
// TODO: 2.3 Player pode descartar cartas e comprar novas
// TODO: 2.4 Player pode competir com a máquina
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
    read_line!();
    // End of Actual code
    Ok(())
}
