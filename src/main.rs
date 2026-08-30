mod hello_game;

use cna::{run, run_for_frames, Result};
use hello_game::HelloGame;

#[derive(Clone, Copy, Debug)]
enum RunMode {
    Interactive,
    Frames(u64),
    ExtensionsSmoke,
}

fn parse_mode() -> core::result::Result<RunMode, String> {
    let mut arguments = std::env::args().skip(1);
    let mut mode = RunMode::Interactive;
    while let Some(argument) = arguments.next() {
        mode = match argument.as_str() {
            "--smoke-test" => RunMode::Frames(60),
            "--extensions-smoke" => RunMode::ExtensionsSmoke,
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

/// Prints what CNA itself says about the runtime it is about to use.
///
/// This is the CNA-only half of the binding: none of it exists in XNA 4.0, so
/// all of it comes from `cna::extensions` rather than from
/// `cna::Microsoft::Xna::Framework`. It is opt-in so the game the template is
/// actually demonstrating stays uncluttered.
fn extensions_smoke() -> Result<()> {
    use cna::extensions::runtime::{
        available_renderers, current_backend_category, current_backend_maturity, current_renderer,
        desktop_operating_system, platform, platform_name, renderer_selection_is_latched,
    };

    println!(
        "cna-rust-template: platform={:?} ({}) os={:?}",
        platform()?,
        platform_name()?,
        desktop_operating_system()?,
    );
    let current = current_renderer()?;
    println!(
        "cna-rust-template: renderer={} category={:?} maturity={:?} latched={}",
        current.name()?,
        current_backend_category()?,
        current_backend_maturity()?,
        renderer_selection_is_latched()?,
    );
    let available = available_renderers()?;
    println!(
        "cna-rust-template: {} renderer identity/identities compiled in",
        available.len()
    );
    for renderer in available {
        println!(
            "cna-rust-template:   {:?} category={:?} maturity={:?}",
            renderer.value(),
            renderer.category()?,
            renderer.maturity()?,
        );
    }
    Ok(())
}

fn execute(mode: RunMode) -> Result<()> {
    match mode {
        RunMode::Interactive => run(HelloGame::new()),
        RunMode::ExtensionsSmoke => extensions_smoke(),
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
