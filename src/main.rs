use std::io;
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use blackjack_engine::game::{Game, GameAction, GameState, RoundOutcome};
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
    let game = Game::new(settings);

    game_loop(game);
}

fn game_loop(mut game: Game) {
    game.shuffle_shoe();
    loop {
        match game.get_state() {
            GameState::WaitingToDeal => {
                println!("Dealing Cards");
                game.deal_initial_cards()
            }
            GameState::WaitingForPlayer { dealer_up_card, player_hand } => {
                println!("\x1B[2J\x1B[1;1H");
                println!("Player's Turn");
                println!("Dealer Showing {}", dealer_up_card.to_string());
                println!("Player Hand {}", player_hand.to_string());
                let next_move = accept_user_input();
                game.process_player_action(next_move);
            }
            GameState::DealerTurn { dealer_hand, player_hand} => {
                println!("\x1B[2J\x1B[1;1H");
                println!("Dealer's Turn");
                println!("Dealer Showing {}", dealer_hand.to_string());
                println!("Player Hand {}", player_hand.to_string());
                sleep(Duration::from_millis(500));
                game.next_dealer_turn();
            }
            GameState::RoundComplete { dealer_hand, player_hand, outcome } => {
                println!("\x1B[2J\x1B[1;1H");
                println!("End of Round");
                println!("Dealer Showing {}", dealer_hand.to_string());
                println!("Player Hand {}", player_hand.to_string());
                match outcome {
                    RoundOutcome::PlayerWin => {println!("Player Wins")}
                    RoundOutcome::DealerWin => {println!("Dealer Wins")}
                    RoundOutcome::Push => {println!("Push")}
                }
                let another = ask_to_continue();
                if another {
                    game.next_round();
                } else {
                    break
                }
            }
            _ => {}
        }
    }
}

fn accept_user_input() -> GameAction {
    println!("Enter your move: (H)it, (S)tand");
    loop{
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        match GameAction::from_string(&user_input) {
            Some(next_move) => {
                return next_move;
            },
            None => {
                println!("Invalid Move, try again");
            }
        }
    }
}

fn ask_to_continue() -> bool {
    println!("Play Again: (Y)es, (N)o");
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        match user_input.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("Invalid input, try again");
                continue;
            }
        }
    }
}