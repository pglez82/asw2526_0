use crate::{
    Coordinates, GameAction, Movement, RandomBot, RenderOptions, YBot, YBotRegistry, game,
};
use crate::{GameStatus, GameY, PlayerId};
use anyhow::Result;
use clap::{Parser, ValueEnum};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::fmt::Display;
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(long_about = "GameY: A command-line implementation of the Game of Y.")]
pub struct CliArgs {
    #[arg(short, long, default_value_t = 7)]
    pub size: u32,

    #[arg(short, long, default_value_t = Mode::Human)]
    pub mode: Mode,

    /// The bot to use (only used with --mode=computer), default = random_bot
    #[arg(short, long, default_value = "random_bot")]
    pub bot: String,

    /// Port to run the server on (only used with --mode=server)
    #[arg(short, long, default_value_t = 3000)]
    pub port: u16,
}

#[derive(Debug, Clone, Copy, ValueEnum, PartialEq)]
pub enum Mode {
    Computer,
    Human,
    Server,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Mode::Computer => "computer",
            Mode::Human => "human",
            Mode::Server => "server",
        };
        write!(f, "{}", s)
    }
}

pub fn run_cli_game() -> Result<()> {
    let args = CliArgs::parse();
    let mut render_options = crate::RenderOptions::default();
    let mut rl = DefaultEditor::new()?;
    let bots_registry = YBotRegistry::new().with_bot(Arc::new(RandomBot));
    let bot: Arc<dyn YBot> = match bots_registry.find(&args.bot) {
        Some(b) => b,
        None => {
            println!(
                "Bot '{}' not found. Available bots: {:?}",
                args.bot,
                bots_registry.names()
            );
            return Ok(());
        }
    };
    let mut game = game::GameY::new(args.size);
    loop {
        println!("{}", game.render(&render_options));
        let status = game.status();
        match status {
            GameStatus::Finished { winner } => {
                println!("Game over! Winner: {}", winner);
                break;
            }
            GameStatus::Ongoing { next_player } => {
                let player = *next_player;
                let prompt = format!(
                    "Current player: {}, action (help = show commands)? ",
                    next_player
                );
                let readline = rl.readline(&prompt);
                match readline {
                    Err(ReadlineError::Interrupted) => {
                        println!("Interrupted");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        continue;
                    }
                    Ok(realine) => {
                        rl.add_history_entry(realine.as_str())?;
                        process_input(
                            &realine,
                            &mut game,
                            &player,
                            &mut render_options,
                            args.mode,
                            bot.as_ref(),
                        )?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn process_input(
    input: &str,
    game: &mut GameY,
    player: &PlayerId,
    render_options: &mut RenderOptions,
    mode: Mode,
    bot: &dyn YBot,
) -> Result<()> {
    let command = parse_command(input, game.total_cells());
    match command {
        Command::Place { idx } => {
            let coords = Coordinates::from_index(idx, game.board_size());
            let movement = Movement::Placement {
                player: *player,
                coords,
            };
            match game.add_move(movement) {
                Ok(()) => {}
                Err(e) => {
                    println!("Error adding move: {}", e);
                }
            }
            if mode == Mode::Computer && !game.check_game_over() {
                if let Some(bot_coords) = bot.choose_move(game) {
                    let bot_movement = Movement::Placement {
                        player: game.next_player().unwrap(),
                        coords: bot_coords,
                    };
                    match game.add_move(bot_movement) {
                        Ok(()) => {}
                        Err(e) => {
                            println!("Error adding bot move: {}", e);
                        }
                    }
                } else {
                    println!("No available moves for the bot.");
                }
            }
        }
        Command::Resign => {
            let movement = Movement::Action {
                player: *player,
                action: GameAction::Resign,
            };
            match game.add_move(movement) {
                Ok(()) => {}
                Err(e) => {
                    println!("Error adding move: {}", e);
                }
            }
        }
        Command::Show3DCoords => {
            render_options.show_3d_coords = !render_options.show_3d_coords;
        }
        Command::ShowIdx => {
            render_options.show_idx = !render_options.show_idx;
        }
        Command::ShowColors => {
            render_options.show_colors = !render_options.show_colors;
        }
        Command::Help => {
            print_help();
        }
        Command::Exit => {
            println!("Exiting the game.");
            std::process::exit(0);
        }
        Command::None => {
            println!("No command entered.");
        }
        Command::Error { message } => {
            println!("Error parsing command: {}", message);
        }
        Command::Save { filename } => {
            let path = std::path::Path::new(&filename);
            game.save_to_file(path)?;
            tracing::info!("Game saved to {}", filename);
        }
        Command::Load { filename } => {
            let path = std::path::Path::new(&filename);
            *game = GameY::load_from_file(path)?;
            tracing::info!("Game loaded from {}", filename);
        }
    }
    Ok(())
}

pub fn parse_command(input: &str, bound: u32) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Command::None;
    }
    match parts[0] {
        "save" => {
            if parts.len() < 2 {
                return Command::Error {
                    message: "Filename required for save command".to_string(),
                };
            }
            Command::Save {
                filename: parts[1].to_string(),
            }
        }
        "load" => {
            if parts.len() < 2 {
                return Command::Error {
                    message: "Filename required for load command".to_string(),
                };
            }
            Command::Load {
                filename: parts[1].to_string(),
            }
        }
        "resign" => Command::Resign,
        "help" => Command::Help,
        "exit" => Command::Exit,
        "show_colors" => Command::ShowColors,
        "show_coords" => Command::Show3DCoords,
        "show_idx" => Command::ShowIdx,
        str => match parse_idx(str, bound) {
            Ok(idx) => Command::Place { idx },
            Err(e) => Command::Error {
                message: format!("Error parsing command: {e}"),
            },
        },
    }
}

fn print_help() {
    println!("Available commands:");
    println!("  <number>        - Place a piece at the specified index number");
    println!("  resign          - Resign from the game");
    println!("  show_coords     - Toggle showing coordinates on the board");
    println!("  show_idx        - Toggle showing index numbers on the board");
    println!("  show_colors     - Toggle showing colors on the board");
    println!("  save <filename> - Save the current game state to a file");
    println!("  load <filename> - Load a game state from a file");
    println!("  exit            - Exit the game");
    println!("  help            - Show this help message");
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Place { idx: u32 },
    Resign,
    None,
    Error { message: String },
    Save { filename: String },
    Load { filename: String },
    Show3DCoords,
    ShowColors,
    ShowIdx,
    Exit,
    Help,
}

pub fn parse_idx(part: &str, bound: u32) -> Result<u32, String> {
    let n = part
        .parse::<u32>()
        .map_err(|_| "Invalid index (not a number)".to_string())?;
    if n >= bound {
        return Err(format!("Index out of bounds: {} > {}", n, bound - 1));
    }
    Ok(n)
}
