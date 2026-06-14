//! Binary entry point: run the factory as a stdio-framed tau extension.
//!
//! The harness launches this with the extension's stdin/stdout wired to the
//! protocol stream. `--workdir <dir>` sets the process working directory, which
//! is the default project repository root when none is given in config.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if let Some(position) = args.iter().position(|arg| arg == "--workdir")
        && let Some(directory) = args.get(position + 1)
    {
        std::env::set_current_dir(directory)?;
    }
    tau_factory::extension::run_stdio()
}
