mod hello_game;

use cna::{run, run_for_frames, Result};
use hello_game::HelloGame;

#[derive(Clone, Copy, Debug)]
enum RunMode {
    Interactive,
    Frames(u64),
}

fn parse_mode() -> core::result::Result<RunMode, String> {
    let mut arguments = std::env::args().skip(1);
    let mut mode = RunMode::Interactive;
    while let Some(argument) = arguments.next() {
        mode = match argument.as_str() {
            "--smoke-test" => RunMode::Frames(60),
            "--stability-test" => RunMode::Frames(600),
            "--frames" => {
                let value = arguments
                    .next()
                    .ok_or("--frames requires a positive integer")?;
                let frames = value
                    .parse::<u64>()
                    .map_err(|_| "--frames requires a positive integer")?;
                if frames == 0 {
                    return Err("--frames requires a positive integer".to_owned());
                }
                RunMode::Frames(frames)
            }
            _ => return Err(format!("unknown argument: {argument}")),
        };
    }
    Ok(mode)
}

fn execute(mode: RunMode) -> Result<()> {
    match mode {
        RunMode::Interactive => run(HelloGame::new()),
        RunMode::Frames(frames) => {
            run_for_frames(HelloGame::new(), frames)?;
            println!(
                "cna-rust-template: completed {frames} real CNA draw frames and shut down cleanly"
            );
            Ok(())
        }
    }
}

fn main() {
    let result = parse_mode()
        .map_err(|message| {
            eprintln!("cna-rust-template: {message}");
            2
        })
        .and_then(|mode| {
            execute(mode).map_err(|error| {
                eprintln!("cna-rust-template: {error}");
                1
            })
        });
    if let Err(code) = result {
        std::process::exit(code);
    }
}
