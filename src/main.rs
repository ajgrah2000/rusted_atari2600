// While in progress, allow dead code (at least until it's all hooked up)
#![allow(dead_code)]
#![allow(unused_variables)]

mod atari2600;

use argh::FromArgs;
use std::str::FromStr;
use strum::IntoEnumIterator;

pub struct Core {}

#[derive(FromArgs)]
/// Rusty Atari 2600 Emulator.
struct RustAtari2600Args {
    /// print PC State Debug Info
    #[argh(switch, short = 'd')]
    debug: bool,

    /// run the emulator with no delay (rather than real-time)
    #[argh(switch, short = 'n')]
    no_delay: bool,

    /// number of clock cycles to stop the emulator (for benchmarking)
    #[argh(option, short = 's')]
    stop_clock: Option<u64>,

    /// run the emulator in full screen mode.
    #[argh(switch, short = 'f')]
    fullscreen: bool,

    /// use PAL palette (instead of NTSC)
    #[argh(switch, short = 'p')]
    pal_palette: bool,

    /// list SDL drivers
    #[argh(switch, short = 'l')]
    list_drivers: bool,

    /// name of cartridge to run
    #[argh(positional)]
    cartridge_name: String,

    /// replay file
    #[argh(option, short = 'r')]
    replay_file: Option<String>,

    /// cartridge type.  (Specifying an invalid option will display available options).
    #[argh(option, short = 'c', default = "atari2600::memory::cartridge::CartridgeType::Default", from_str_fn(parse_cartridge))]
    cartridge_type: atari2600::memory::cartridge::CartridgeType,
}

fn cartridge_type_help_fn() -> String {
    atari2600::memory::cartridge::CartridgeType::iter().fold("cartridge type: ".to_owned(), |all, value| format!("{} {:?}", all, value))
}
fn parse_cartridge(value: &str) -> Result<atari2600::memory::cartridge::CartridgeType, String> {
    match atari2600::memory::cartridge::CartridgeType::from_str(value) {
        Ok(x) => Ok(x),
        Err(x) => Err(format!("Supplied {}. Error: {}\n{}", value, x, cartridge_type_help_fn())),
    }
}

fn full_description_string() -> String {
    let mut description = "Possible audio drivers, to use prefix command with: SDL_AUDIODRIVER=<driver>\n".to_owned();
    description += &sdl2::audio::drivers().map(|s| s.to_string()).reduce(|cur: String, nxt: String| cur + ", " + &nxt).unwrap();
    description += "\n";
    description += "Possible video drivers, to use prefix command with: SDL_VIDEODRIVER=<driver>\n";
    description += &sdl2::video::drivers().map(|s| s.to_string()).reduce(|cur: String, nxt: String| cur + ", " + &nxt).unwrap();
    description += "\n";

    description.to_string()
}

fn main() {
    let args: RustAtari2600Args = argh::from_env();

    if args.list_drivers {
        println!("{}", full_description_string());
    }

    let mut atari_machine = atari2600::atari2600::Atari2600::new(args.debug, !args.no_delay, args.stop_clock.unwrap_or(0), args.cartridge_name, args.cartridge_type, args.fullscreen, args.pal_palette);

    atari_machine.power_atari2600();

    println!("Finished.");
}
