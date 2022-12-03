/// Initializes the logger.
pub fn log_init() {
    env_logger::init();
    log::info!(
        "Logger initialized at max level set to {}",
        log::max_level()
    );
}
