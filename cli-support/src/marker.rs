/// This guard tells the marco that the application is being compiled with a CLI that supports assets
///
/// *If you do not hold this marker when compiling an application that uses the assets macro, the macro will display a warning about asset support*
pub struct ManganisSupportGuard(());

impl ManganisSupportGuard {
    /// Creates a new marker
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for ManganisSupportGuard {
    fn default() -> Self {
        std::env::set_var("MANGANIS_SUPPORT", "true");
        Self(())
    }
}

impl Drop for ManganisSupportGuard {
    fn drop(&mut self) {
        std::env::remove_var("MANGANIS_SUPPORT");
    }
}
