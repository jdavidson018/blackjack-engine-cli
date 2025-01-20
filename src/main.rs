use std::io;
use std::thread::sleep;
use std::time::Duration;
use clap::Parser;
use blackjack_engine::game::{Game, GameAction, GameState};
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
            GameState::WaitingForBet { player_bankroll } => {
                println!("\x1B[2J\x1B[1;1H");
                println!("Bankroll: ${}", player_bankroll);
                println!("Place your bet");
                game.accept_user_bet(accept_user_bet());
            }
            GameState::WaitingToDeal { player_bet, player_bankroll } => {
                print_game_state(Some(player_bankroll), None, None, None);
                game.deal_initial_cards()
            }
            GameState::PlayerTurn { dealer_hand, player_hands, player_bankroll, active_hand_index } => {
                print_game_state(Some(player_bankroll),
                                 Some(&Hand::with_card(dealer_hand.cards[0].clone())),
                                 Some(player_hands),
                                 Option::from(*active_hand_index));
                let next_move = accept_user_input();
                game.process_player_action(next_move, *active_hand_index);
            }
            GameState::DealerTurn { dealer_hand, player_hands, player_bankroll } => {
                print_game_state(Some(player_bankroll),
                                 Some(dealer_hand),
                                 Some(player_hands),
                                 None);
                sleep(Duration::from_millis(500));
                game.next_dealer_turn();
            }
            GameState::RoundComplete { dealer_hand, player_hands, player_bankroll } => {
                print_game_state(Some(player_bankroll),
                                 Some(dealer_hand),
                                 Some(player_hands),
                                 None);

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
                    dealer_hand: Option<&Hand>,
                    player_hands: Option<&Vec<Hand>>,
                    active_hand_index: Option<usize>
) {
    println!("\x1B[2J\x1B[1;1H");

    println!("bankroll: ${}",
             player_bankroll.unwrap_or(&0.0));
    println!();

    println!("Dealer Cards:");
    println!("{}", dealer_hand.map_or("No cards".to_string(), |h| h.to_string()));
    println!();

    println!("Player:");
    match player_hands {
        Some(hands) => {
            if hands.is_empty() {
                println!("No cards");
            } else {
                // Print each hand on a new line with appropriate indexing
                for (i, hand) in hands.iter().enumerate() {
                    println!("Hand {}:", i+1);
                    println!("Bet ${}", hand.bet);
                    match &hand.outcome {
                        Some(outcome) => {
                            match active_hand_index {
                                Some(index) => {
                                    if i == index {
                                        println!("Cards {} - {} <", hand.to_string(), outcome.to_string())
                                    } else {
                                        println!("Cards {} - {}", hand.to_string(), outcome.to_string())
                                    }
                                },
                                None => println!("Cards {} - {}", hand.to_string(), outcome.to_string())
                            }
                        },
                        None => {
                            match active_hand_index {
                                Some(index) => {
                                    if i == index {
                                        println!("Cards {} <", hand.to_string())
                                    } else {
                                        println!("Cards {}", hand.to_string())
                                    }
                                },
                                None => {
                                    println!("Cards {}", hand.to_string())
                                }
                            }
                        },
                    }
                    println!();
                }
            }
        }
        None => println!("No cards"),
    }
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
    println!("Enter your move: (H)it, (S)tand, (D)ouble, S(P)lit");
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

enum Turn {
    DealerTurn,
    PLayerTurn(i32)
}