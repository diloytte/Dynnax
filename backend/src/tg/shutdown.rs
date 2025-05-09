#[cfg(all(not(feature = "remote"), not(feature = "production")))]
pub fn maybe_shutdown_command(text: &str) {
    if text.contains("/shutdown_secret123") {
        std::process::exit(0);
    }
}
