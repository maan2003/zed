//! Binary entry point: run the factory as a stdio-framed tau extension.
//!
//! The harness launches this with the extension's stdin/stdout wired to the
//! protocol stream. Project-global settings, such as the repository root, come
//! from the harness `Configure` message rather than ad-hoc process arguments.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tau_factory::extension::run_stdio()
}
