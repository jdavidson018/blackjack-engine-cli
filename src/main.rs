use std::io;
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use blackjack_engine::game::{Game, GameAction, GameState, RoundOutcome};
use blackjack_engine::game_settings::GameSettings;
use blackjack_engine::hand::Hand;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    // Name of non-npc player
    #[arg(short = 'n', long = "name")]
    player_name: String,

    // Number of decks in the shoe
    #[arg(short, long, default_value_t = 1)]
    deck_count: u8,
}

fn main() {
    let args = Args::parse();

    println!("Welcome to Blackjack, {}!", args.player_name);
    println!(
        "Starting a game with {} decks",
        args.deck_count
    );

    let settings = GameSettings::new(args.player_name, args.deck_count);
    let game = Game::new(settings);

    game_loop(game);
}

fn game_loop(mut game: Game) {
    game.shuffle_shoe();
    loop {
        match game.get_state() {
            GameState::WaitingForBet => {
                println!("Place your bet");
                game.accept_user_bet(accept_user_bet());
            }
            GameState::WaitingToDeal { player_bet, player_bankroll } => {
                print_game_state(Some(player_bankroll), Some(player_bet), None, None);
                game.deal_initial_cards()
            }
            GameState::WaitingForPlayer { dealer_hand, player_hand, player_bankroll } => {
                print_game_state(Some(player_bankroll),
                                 Some(&player_hand.bet),
                                 Some(&Hand::with_card(dealer_hand.cards[0].clone())),
                                 Some(player_hand));
                let next_move = accept_user_input();
                game.process_player_action(next_move);
            }
            GameState::DealerTurn { dealer_hand, player_hand, player_bankroll } => {
                print_game_state(Some(player_bankroll),
                                 Some(&player_hand.bet),
                                 Some(dealer_hand),
                                 Some(player_hand));
                sleep(Duration::from_millis(500));
                game.next_dealer_turn();
            }
            GameState::RoundComplete { dealer_hand, player_hand, outcome, player_bankroll } => {
                print_game_state(Some(player_bankroll),
                                 Some(&player_hand.bet),
                                 Some(dealer_hand),
                                 Some(player_hand));
                match outcome {
                    RoundOutcome::PlayerWin => { println!("Player Wins") }
                    RoundOutcome::DealerWin => { println!("Dealer Wins") }
                    RoundOutcome::Push => { println!("Push") }
                }
                let another = ask_to_continue();
                if another {
                    game.next_round();
                } else {
                    break;
                }
            }
            _ => {}
        }
    }
}

fn print_game_state(player_bankroll: Option<&f64>,
                    player_bet: Option<&f64>,
                    dealer_hand: Option<&Hand>,
                    player_hand: Option<&Hand>
) {
    println!("\x1B[2J\x1B[1;1H");
    println!("bank: ${}, bet: ${}",
             player_bankroll.unwrap_or(&0.0),
             player_bet.unwrap_or(&0.0));
    println!("Dealer: {}", dealer_hand.map_or("No cards".to_string(), |h| h.to_string()));
    println!();
    println!("Player: {}", player_hand.map_or("No cards".to_string(), |h| h.to_string()));
}

fn accept_user_bet() -> f64 {
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        // Parse the input, trim whitespace and newlines first
        match user_input.trim().parse::<f64>() {
            Ok(bet) => return bet,
            Err(_) => println!("Please enter a valid positive number")
        }
    }
}

fn accept_user_input() -> GameAction {
    println!("Enter your move: (H)it, (S)tand");
    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        match GameAction::from_string(&user_input) {
            Some(next_move) => {
                return next_move;
            }
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