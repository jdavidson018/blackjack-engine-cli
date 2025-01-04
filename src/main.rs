use std::io;
use std::io::Write;
use clap::Parser;
use blackjack_engine::game::Game;
use blackjack_engine::game_settings::GameSettings;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Name of non-npc player
    #[arg(short = 'n', long = "name")]
    player_name: String,

    // Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    deck_count: u8,

    // Number of Non-Dealer Players
    #[arg(short, long, default_value_t = 1)]
    player_count: u8,
}
fn main() {
    let args = Args::parse();

    println!("Welcome to Blackjack, {}!", args.player_name);
    println!(
        "Starting a game with {} players and {} decks",
        args.player_count, args.deck_count
    );

    let settings = GameSettings::new(args.player_name, args.deck_count, args.player_count);
    let mut game = Game::new(settings);

    loop {
        game.play_round();

        print!("\nWould you like to play another round? (Y/N) ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() != "y" {
            println!("Thanks for playing!");
            break;
        }
    }
}