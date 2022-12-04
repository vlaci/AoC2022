use color_eyre::{
    eyre::{ContextCompat, WrapErr},
    Result,
};

pub fn init() -> Result<String> {
    std::env::set_var("RUST_BACKTRACE", "full");
    color_eyre::install()?;

    let path = std::env::args()
        .nth(1)
        .wrap_err("Missing argument: <input file>")?;
    std::fs::read_to_string(&path).wrap_err_with(|| format!("Failed to read from {:?}", path))
}
