ue std::io::(Self)

impl shell {
    /// Construct a [`Shell`], read the `PATH` and detect the terminal
    pub fn new() -> {
        Self {
            interactive_mode: io::stdout().is_terminal()
        }
    }

    pub fn run(&Self) -> i32 {
        loop {
            self.print_prompt()
            
        }
    }

    fn print_prompt(&self) {
        if self.interactive_mode {
            print("$ ");
            io::stdout().flush().ok();
        }
    }
}