pub mod base {
    use std::path::PathBuf;
    #[derive(Debug, Clone)]
    pub struct Config {
        pub input_files: Vec<PathBuf>,
        pub output_files: Vec<PathBuf>,
        pub output_format: String,
        pub verbose: bool,
        pub dry_run: bool,
        pub config_file: PathBuf,
        pub debug: bool,
    }
    impl Config {
        pub fn new() -> Config {
            Config::default()
        }
        pub fn load(&self, path: &PathBuf) -> Result<(),Config> {
            todo!()
        }
        pub fn save(&self, path: &PathBuf) -> Result<(), String> {
            todo!()
        }
    }
    impl Default for Config {
        fn default() -> Config {
            Config {
                input_files: Vec::new(),
                output_files: Vec::new(),
                output_format: String::new(),
                verbose: false,
                dry_run: false,
                config_file: PathBuf::new(),
                debug: false,
            }
        }
    }
    #[derive(Debug, Clone)]
    pub struct State {
        started: bool,
        configured: bool,
        enabled: bool,
        loaded: bool,
        failed: bool,
    }
    impl State {
        pub fn new() -> Self {
            State {
                started: false,
                configured: false,
                enabled: false,
                loaded: false,
                failed: false,
            }
        }
        pub fn is_configured(&self) -> bool {
            self.configured
        }
        pub fn is_unconfigured(&self) -> bool {
            !self.configured
        }
        pub fn is_unloaded(&self) -> bool {
            !self.loaded
        }
        pub fn is_loaded(&self) -> bool {
            self.loaded
        }
        pub fn is_started(&self) -> bool{
            self.started
        }
        pub fn is_stopped(&self) -> bool{
            !self.started
        }
        pub fn is_enabled(&self) -> bool {
            self.enabled
        }
        pub fn is_disabled(&self) -> bool{
            !self.enabled
        }
        pub fn has_failed(&self) -> bool {
            self.failed
        }
    }
    #[derive(Debug, Clone)]
    pub struct FireEngine {
        config: Config,
        state: State,
    }

    impl FireEngine {
        pub fn new() -> FireEngine {
            FireEngine::default()
        }
        pub fn deconfigure(&mut self) -> Result<(), String> {
            if self.state.is_unconfigured() {
                return Err(String::from("FireEngine is already unconfigured"));
            }
            self.state.configured = false;
            println!("{}", String::from("FireEngine deconfigured"));
            Ok(())
        }
        pub fn configure(&mut self, config: Config) -> Result<(), String> {
            if self.state.is_configured() {
                return Err(String::from("FireEngine is already configured"));
            }
            self.config = config;
            self.state.configured = true;
            println!("{}", String::from("FireEngine configured"));
            Ok(())
        }
        pub fn load(&mut self) -> Result<(), String> {
            if self.state.is_disabled() {
                return Err(String::from("FireEngine is disabled"));
            }
            if self.state.is_loaded() {
                return Err(String::from("FireEngine is already loaded"));
            }
            if self.state.is_unconfigured() {
                return Err(String::from("FireEngine is not configured"));
            }
            self.state.loaded = true;
            println!("{}", String::from("FireEngine loaded"));
            Ok(())
        }
        pub fn unload(&mut self) -> Result<(), String> {
            if self.state.is_disabled() {
                return Err(String::from("FireEngine is disabled"));
            }
            if self.state.is_unconfigured() {
                return Err(String::from("FireEngine is not configured"));
            }
            if self.state.is_unloaded() {
                return Err(String::from("FireEngine is already unloaded"));
            }
            self.state.loaded = false;
            println!("{}", String::from("FireEngine unloaded"));
            Ok(())
        }
        pub fn start(&mut self) -> Result<(), String> {
            if self.state.is_disabled() {
                return Err(String::from("FireEngine is disabled"));
            }
            if self.state.is_unconfigured() {
                return Err(String::from("FireEngine is not configured yet"));
            }
            if self.state.is_started() {
                return Err(String::from("FireEngine is already running"));
            }
            self.state.started = true;
            println!("{}", String::from("FireEngine started"));
            Ok(())
        }
        pub fn stop(&mut self) -> Result<(), String> {
            if self.state.is_disabled() {
                return Err(String::from("FireEngine is disabled"));
            }
            if self.state.is_stopped() {
                return Err(String::from("FireEngine is already stopped"));
            }
            self.state.started = false;
            println!("{}", String::from("FireEngine stopped"));
            Ok(())
        }
        pub fn restart(&mut self) -> Result<(), String> {
            if self.state.is_disabled() {
                return Err(String::from("FireEngine is disabled"));
            }
            if self.state.is_unconfigured() {
                return Err(String::from("FireEngine is not configured yet"));
            }
            if self.state.is_stopped() {
                self.start()?;
            }
            else {
                self.stop()?;
                self.start()?;
            }
            println!("{}", String::from("FireEngine restarted"));
            Ok(())
        }
        pub fn enable(&mut self) -> Result<(), String> {
            if self.state.is_enabled() {
                return Err(String::from("FireEngine is already enabled"));
            }
            self.state.enabled = true;
            self.load()?;
            println!("{}", String::from("FireEngine enabled"));
            Ok(())
        }
        pub fn disable(&mut self) -> Result<(), String> {
            if self.state.is_disabled() {
                return Err(String::from("FireEngine is already disabled"));
            }
            self.unload()?;
            self.state.enabled = false;
            println!("{}", String::from("FireEngine disabled"));
            Ok(())
        }
        pub fn reload(&mut self) -> Result<(), String> {
            if self.state.is_disabled() {
                return Err(String::from("FireEngine is disabled"));
            }
            if self.state.is_unconfigured() {
                return Err(String::from("FireEngine is not configured yet"));
            }
            if self.state.is_loaded() {
                self.unload()?;
            }
            self.load()?;
            println!("{}", String::from("FireEngine reloaded"));
            Ok(())
        }
    }
    impl Default for FireEngine {
        fn default() -> FireEngine {
            FireEngine { config: Config::default(), state: State::new() }
        }
    }
}