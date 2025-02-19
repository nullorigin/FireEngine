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
        pub fn load(&self, path: &PathBuf) -> Result<(), String> {
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
    pub struct Table {
        pub table: Vec<Vec<String>>,
        pub row: usize,
        pub column: usize,
    }
    impl Table {
        pub fn new() -> Self {
            Table {
                table: Vec::<Vec<String>>::new(),
                row: 0,
                column: 0,
            }
        }
        pub fn reset(&mut self) {
            self.table = Vec::new();
            self.row = 0;
            self.column = 0;
        }
        pub fn get_cell(&self) -> String {
            self.table[self.row][self.column].clone()
        }
        pub fn set_cell(&mut self, value: String) -> Result<(), String> {
            if self.row >= self.table.len() || self.column >= self.table[self.row].len() {
                return Err(String::from("Cell out of bounds"));
            }
            self.table[self.row][self.column] = value;
            Ok(())
        }
        pub fn get_column(&self) -> Vec<String> {
            self.table
                .iter()
                .map(|row| row[self.column].clone())
                .collect()
        }
        pub fn set_column(&mut self, value: Vec<String>) -> Result<(), String> {
            if value.len() == 0 || self.column >= self.table[self.row].len() {
                return Err(String::from("Column cannot be empty"));
            }
            for (i, row) in self.table.iter_mut().enumerate() {
                row[self.column] = value[i].clone();
            }
            Ok(())
        }
        pub fn get_row(&self) -> Vec<String> {
            self.table[self.row].clone()
        }
        pub fn set_row(&mut self, value: Vec<String>) -> Result<(), String> {
            if value.len() == 0 || self.row >= self.table.len() {
                return Err(String::from("Row cannot be empty"));
            }
            self.table[self.row] = value;
            Ok(())
        }
        pub fn get_table(&self) -> Vec<Vec<String>> {
            self.table.clone()
        }
        pub fn set_table(&mut self, value: Vec<Vec<String>>) -> Result<(), String> {
            if value.len() == 0
                || value[0].len() == 0
                || self.table.len() == 0
                || self.table[0].len() == 0
            {
                return Err(String::from("Table cannot be empty"));
            }
            self.table = value;
            Ok(())
        }
        pub fn goto_next_cell(&mut self) -> Result<(), String> {
            if self.column < self.table[self.row].len() - 1 {
                self.column += 1;
            } else if self.row < self.table.len() - 1 {
                self.row += 1;
                self.column = 0;
            } else {
                return Err(String::from("Cell out of bounds"));
            }
            Ok(())
        }
        pub fn goto_previous_cell(&mut self) -> Result<(), String> {
            if self.column > 0 {
                self.column -= 1;
            } else if self.row > 0 {
                self.row -= 1;
                self.column = self.table[self.row].len() - 1;
            } else {
                return Err(String::from("Cell out of bounds"));
            }
            Ok(())
        }
        pub fn goto_next_row(&mut self) -> Result<(), String> {
            let row = (self.row as isize) + 1;
            let column = self.column as isize;
            if row >= self.table.len() as isize {
                self.row = 0;
                if column >= self.table[self.row].len() as isize {
                    self.column = 0;
                } else {
                    self.column += 1;
                }
            }
            Ok(())
        }
        pub fn goto_previous_row(&mut self) -> Result<(), String> {
            let row = (self.row as isize) - 1;
            let column = self.column as isize;
            if row < 0 {
                self.row = self.table.len() - 1;
                if column < 0 {
                    self.column = self.table[self.row].len() - 1;
                } else {
                    self.column += 1;
                }
            }
            Ok(())
        }
        pub fn goto_next_column(&mut self) -> Result<(), String> {
            let column = (self.column as isize) + 1;
            let row = self.row as isize;
            if column >= self.table[self.row].len() as isize {
                self.column = 0;
                if row >= self.table.len() as isize {
                    self.row = 0;
                } else {
                    self.row += 1;
                }
            }
            Ok(())
        }
        pub fn goto_previous_column(&mut self) -> Result<(), String> {
            let column = (self.column as isize) - 1;
            let row = self.row as isize;
            if column < 0 {
                self.column = self.table[self.row].len() - 1;
                if row < 0 {
                    self.row = self.table.len() - 1;
                } else {
                    self.row += 1;
                }
            }
            Ok(())
        }
        pub fn goto_cell(&mut self, row: usize, column: usize) -> Result<(), String> {
            if row >= self.table.len() || column >= self.table[row].len() {
                return Err(String::from("Cell out of bounds"));
            }
            self.row = row;
            self.column = column;
            Ok(())
        }
        pub fn goto_row(&mut self, row: usize) -> Result<(), String> {
            if row >= self.table.len() {
                return Err(String::from("Row out of bounds"));
            }
            self.row = row;
            Ok(())
        }
        pub fn goto_column(&mut self, column: usize) -> Result<(), String> {
            if column >= self.table[self.row].len() {
                return Err(String::from("Column out of bounds"));
            }
            self.column = column;
            Ok(())
        }
        pub fn replace(&self, value: &str, replacement: &str) -> Vec<Vec<String>> {
            let mut new_table = self.table.clone();
            for row in new_table.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = cell.replace(value, replacement);
                }
            }
            new_table
        }
        pub fn replace_cell(
            &self,
            value: &str,
            replacement: &str,
            row: usize,
            column: usize,
        ) -> Vec<Vec<String>> {
            let mut new_table = self.table.clone();
            new_table[row][column] = new_table[row][column].replace(value, replacement);
            new_table
        }
        pub fn replace_row(&self, value: &str, replacement: &str, row: usize) -> Vec<Vec<String>> {
            let mut new_table = self.table.clone();
            for cell in new_table[row].iter_mut() {
                *cell = cell.replace(value, replacement);
            }
            new_table
        }
        pub fn replace_column(
            &self,
            value: &str,
            replacement: &str,
            column: usize,
        ) -> Vec<Vec<String>> {
            let mut new_table = self.table.clone();
            for row in new_table.iter_mut() {
                row[column] = row[column].replace(value, replacement);
            }
            new_table
        }
        pub fn resize_rows(&mut self, rows: usize) -> Result<(), String> {
            if rows == 0 {
                return Err(String::from("Rows cannot be empty"));
            }
            let mut new_table = self.table.clone();
            if new_table.len() < rows {
                for i in 0..(rows - new_table.len()) {
                    new_table.push(vec!["".to_string(); self.table[i].len()]);
                }
            } else if new_table.len() > rows {
                new_table.truncate(rows);
            }
            Ok(())
        }
        pub fn resize_columns(&mut self, columns: usize) -> Result<(), String> {
            if columns == 0 {
                return Err(String::from("Columns cannot be empty"));
            }
            let mut new_table = self.table.clone();
            for row in new_table.iter_mut() {
                if row.len() < columns {
                    for i in 0..(columns - row.len()) {
                        row.push("".to_string());
                    }
                } else if row.len() > columns {
                    row.truncate(columns);
                }
            }
            Ok(())
        }
        pub fn fill_table(&mut self, values: String, delimiter: char) -> Result<(), String> {
            if self.table.len() == 0 || self.table[0].len() == 0 {
                return Err(String::from("Table cannot be empty"));
            }
            let mut i = 0;
            for row in self.table.iter_mut() {
                for cell in row.iter_mut() {
                    *cell = values.split(delimiter).collect::<Vec<&str>>()[i].to_string();
                }
                i += 1;
            }
            Ok(())
        }
        pub fn clear(&mut self) {
            self.table = Vec::<Vec<String>>::new();
        }
        pub fn clear_row(&mut self, row: usize) -> Result<(), String> {
            if row >= self.table.len() {
                return Err(String::from("Row out of bounds"));
            }
            self.table[row] = Vec::<String>::new();
            Ok(())
        }
        pub fn clear_column(&mut self, column: usize) -> Result<(), String> {
            if column >= self.table[self.row].len() {
                return Err(String::from("Column out of bounds"));
            }
            for row in self.table.iter_mut() {
                row[column] = "".to_string();
            }
            Ok(())
        }
        pub fn is_empty(&self) -> bool {
            self.table.len() == 0 || self.table[0].len() == 0
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct State {
        pub enabled: bool,
        pub configured: bool,
        pub loaded: bool,
        pub started: bool,
        pub failures: usize,
        pub debug: bool,
    }

    impl State {
        pub fn new() -> Self {
            State {
                enabled: false,
                configured: false,
                loaded: false,
                started: false,
                failures: 0,
                debug: false,
            }
        }

        pub fn on_enable(&mut self) -> Result<(), String> {
            if self.enabled {
                self.on_fail();
                return Err(String::from("FireEngine is already enabled"));
            } else if self.debug {
                self.on_success();
                println!("{}", String::from("FireEngine enabled"));
            }
            self.enabled = true;
            Ok(())
        }
        pub fn on_disable(&mut self) -> Result<(), String> {
            if !self.enabled {
                self.on_fail();
                return Err(String::from("FireEngine is not enabled"));
            } else if self.debug {
                self.on_success();
                println!("{}", String::from("FireEngine disabled"));
            }
            self.enabled = false;
            Ok(())
        }
        pub fn on_config(&mut self) -> Result<(), String> {
            if self.configured {
                self.on_fail();
                return Err(String::from("FireEngine is already configured"));
            } else if self.debug {
                self.on_success();
                println!("{}", String::from("FireEngine configured"));
            }
            self.configured = true;
            Ok(())
        }
        pub fn on_deconfig(&mut self) -> Result<(), String> {
            if !self.configured {
                self.on_fail();
                return Err(String::from("FireEngine is not configured"));
            } else if self.debug {
                self.on_success();
                println!("{}", String::from("FireEngine deconfigured"));
            }
            self.configured = false;
            Ok(())
        }
        pub fn on_load(&mut self) -> Result<(), String> {
            if self.loaded {
                self.on_fail();
                return Err(String::from("FireEngine is already loaded"));
            } else if self.debug {
                self.on_success();
                println!("{}", String::from("FireEngine loaded"));
            }
            self.loaded = true;
            Ok(())
        }
        pub fn on_unload(&mut self) -> Result<(), String> {
            if !self.loaded {
                self.on_fail();
                return Err(String::from("FireEngine is not loaded"));
            } else if self.debug {
                self.on_success();
                println!("{}", String::from("FireEngine unloaded"));
            }
            self.loaded = false;
            Ok(())
        }
        pub fn on_start(&mut self) -> Result<(), String> {
            if self.started {
                self.on_fail();
                return Err(String::from("FireEngine is already started"));
            } else if self.debug {
                self.on_success();
                println!("{}", String::from("FireEngine started"));
            }
            self.started = true;
            Ok(())
        }
        pub fn on_stop(&mut self) -> Result<(), String> {
            if !self.started {
                self.on_fail();
                return Err(String::from("FireEngine is already stopped"));
            } else if self.debug {
                self.on_success();
                println!("{}", String::from("FireEngine stopped"));
            }
            self.started = false;
            Ok(())
        }
        pub fn on_fail(&mut self) -> Result<(), String> {
            if self.debug {
                println!("{}", String::from("FireEngine failed"));
            }
            self.failures += 1;
            Ok(())
        }
        pub fn on_success(&mut self) -> Result<(), String> {
            if self.failures > 0 && self.debug {
                println!(
                    "{}{}{}",
                    String::from("FireEngine recovered from "),
                    self.failures.to_string(),
                    String::from(" failures")
                );
            }
            self.failures = 0;
            Ok(())
        }
    }
    #[derive(Debug, Clone)]
    pub struct Daemon {
        config: Config,
        state: State,
    }

    impl Daemon {
        pub fn new() -> Daemon {
            Daemon {
                config: Config::new(),
                state: State::new(),
            }
        }
        pub fn debug_on(&mut self) {
            self.state.debug = true;
            self.config.debug = true;
        }
        pub fn debug_off(&mut self) {
            self.state.debug = false;
            self.config.debug = false;
        }
        pub fn enable(&mut self) -> Result<(), String> {
            self.state.on_enable()
        }
        pub fn config(&mut self) -> Result<(), String> {
            self.state.on_config()
        }
        pub fn load(&mut self) -> Result<(), String> {
            self.state.on_load()
        }
        pub fn start(&mut self) -> Result<(), String> {
            self.state.on_start()
        }
        pub fn stop(&mut self) -> Result<(), String> {
            self.state.on_stop()
        }
        pub fn unload(&mut self) -> Result<(), String> {
            self.state.on_unload()
        }
        pub fn deconfig(&mut self) -> Result<(), String> {
            self.state.on_deconfig()
        }
        pub fn disable(&mut self) -> Result<(), String> {
            self.state.on_disable()
        }
        pub fn fail(&mut self) -> Result<(), String> {
            self.state.on_fail()
        }
        pub fn success(&mut self) -> Result<(), String> {
            self.state.on_success()
        }
    }
    #[derive(Debug, Clone)]
    pub struct FireEngine {
        pub daemon: Daemon,
        pub table: Vec<Vec<String>>,
    }

    impl FireEngine {
        pub fn new() -> FireEngine {
            FireEngine::default()
        }
    }
    impl Default for FireEngine {
        fn default() -> FireEngine {
            FireEngine {
                daemon: Daemon::new(),
                table: Vec::<Vec<String>>::new(),
            }
        }
    }
}
