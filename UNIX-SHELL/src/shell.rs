use std::io::(Self)
use crate::error::Error;
use crate::path::PathResolver;

/// The shell's read-eval-print loop and all state it needs to run it.
///
/// Constructed once in `main`. Owns the [`PathResolver`] so `PATH` is
/// read from the environment exactly once for the lifetime of the shell.

pub struct Shell {
    /// pre-built for reuse for every external lookup
    resolver: PathResolver,
    /// `true` when stdout is an interactive terminal; supresses promt otherwise
    interactive_mode: bool,
}
impl Shell {
    /// Construct a [`Shell`], read the `PATH` and detect the terminal
    pub fn new() -> {
        Self {
            resolver: PathResolver::new(),
            interactive_mode: io::stdout().is_terminal()
        }
    }

    /// Read command, execute it, print result, continue the loop until EOF
    /// --returning the process exit code to `main`
    /// 
    /// Returning an exit code rather than calling  `std::process::exit` helps
    /// -- keeps this function clean and lets `main` perform any future teardown
    pub fn run(&Self) -> i32 {
        let stdin = io::stdin.lock();

        loop {
            self.print_prompt();
            let mut line = String::new();
            match stdin.read_line(&mut line) {
                Ok(0) => {
                    // EOF: ctr-D in interactive mode, or end of a piped script
                    println();
                }
                return 0;
            }
            Ok(_) => {}
            Err(e) => { eprintln!("rush: read erro: {e}");
                return 1;
            }
        }

        match Command::parse(&line) {
            // if user readline reads whitespace
            Err(Error::EmptyInput) => continue,

            Err(e) => eprintln!("{e}"),
            Ok(cmd) => {
                match cmd.run(&self.resolver) {
                    Err(e) => eprintln!("{e}"),
                    ok(_) => {}
                }
            }
        }
    }

    fn print_prompt(&self) {
        if self.interactive_mode {
            print("$ ");
            io::stdout().flush().ok();
        }
    }
}