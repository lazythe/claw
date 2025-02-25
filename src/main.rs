mod builtins;
mod command;
mod inout;
mod completion;

use users::get_current_username;
use colored::*;
use rustyline::error::ReadlineError;
use rustyline::{Config, Editor, Helper};
use std::env;
use rustyline::validate::{Validator, ValidationResult, ValidationContext};
use rustyline::highlight::{Highlighter, MatchingBracketHighlighter};
use rustyline::hint::{Hinter, HistoryHinter};
use completion::FileCompleter;
use std::io::{self, Write};

struct ShellHelper {
    completer: FileCompleter,
    highlighter: MatchingBracketHighlighter,
    hinter: HistoryHinter,
}

impl Helper for ShellHelper {}

impl rustyline::completion::Completer for ShellHelper {
    type Candidate = rustyline::completion::Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &rustyline::Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let (start, candidates) = self.completer.complete(line, pos, ctx)?;
        
        if candidates.len() > 1 {
            let term_width = term_size::dimensions().map(|(w, _)| w).unwrap_or(80);
            
            let max_width = candidates.iter()
                .map(|c| c.display.len())
                .max()
                .unwrap_or(0) + 2;
            
            let cols = term_width / max_width;
            if cols > 0 {
                print!("\x1B[s\n");
                io::stdout().flush().ok();
                
                let mut current_col = 0;
                for candidate in &candidates {
                    print!("{:<width$}", candidate.display, width = max_width);
                    current_col += 1;
                    
                    if current_col >= cols {
                        println!();
                        current_col = 0;
                    }
                }
                
                if current_col > 0 {
                    println!();
                }
                
                print!("\x1B[u");
                io::stdout().flush().ok();
            }
        }
        
        Ok((start, candidates))
    }
}

impl Hinter for ShellHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for ShellHelper {
    fn highlight_prompt<'b, 's: 'b, 'p: 'b>(
        &'s self,
        prompt: &'p str,
        default: bool,
    ) -> std::borrow::Cow<'b, str> {
        if default {
            std::borrow::Cow::Borrowed(prompt)
        } else {
            std::borrow::Cow::Owned(prompt.to_owned())
        }
    }

    fn highlight_hint<'h>(&self, hint: &'h str) -> std::borrow::Cow<'h, str> {
        std::borrow::Cow::Owned(hint.to_owned())
    }

    fn highlight<'l>(&self, line: &'l str, pos: usize) -> std::borrow::Cow<'l, str> {
        self.highlighter.highlight(line, pos)
    }

    fn highlight_char(&self, line: &str, pos: usize) -> bool {
        self.highlighter.highlight_char(line, pos)
    }
}

impl Validator for ShellHelper {
    fn validate(
        &self,
        _ctx: &mut ValidationContext,
    ) -> rustyline::Result<ValidationResult> {
        Ok(ValidationResult::Valid(None))
    }
}

fn main() {
    let config = Config::builder()
        .auto_add_history(true)
        .completion_type(rustyline::CompletionType::Circular)
        .build();
    
    let helper = ShellHelper {
        completer: FileCompleter::new(),
        highlighter: MatchingBracketHighlighter::new(),
        hinter: HistoryHinter {},
    };
    
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(helper));
    
    if rl.load_history("history.txt").is_err() {
        println!("{}", "No previous history.".yellow());
    }

    loop {
        let username = get_current_username().unwrap();
        let prompt = format!("[{}] {} {}> ", username.to_string_lossy().bright_green(), env::current_dir().unwrap().display().to_string().bright_blue(), "~".bright_blue());
        
        match rl.readline(&prompt) {
            Ok(line) => {
                let _ = rl.add_history_entry(line.as_str());
                
                let input = line.trim();
                if input.is_empty() {
                    continue;
                }

                if input.contains(">") || input.contains("<") {
                    handle_redirection(input);
                } else if input.contains("|") {
                    handle_pipe(input);
                } else {
                    let tokens: Vec<&str> = input.split_whitespace().collect();
                    if tokens.is_empty() {
                        continue;
                    }

                    let command = tokens[0];
                    let args = &tokens[1..];

                    if builtins::is_builtin(command) {
                        builtins::execute_builtin(command, args);
                    } else {
                        command::execute_command(command, args);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "^C".red());
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "exit".bright_yellow());
                break;
            }
            Err(err) => {
                eprintln!("{}", format!("Error: {}", err).red());
                break;
            }
        }
    }

    if let Err(err) = rl.save_history("history.txt") {
        eprintln!("{}", format!("Error saving history: {}", err).red());
    }
}

fn handle_redirection(input: &str) {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if let Some(output_idx) = parts.iter().position(|&x| x == ">") {
        let (command_args, output_file) = parts.split_at(output_idx);
        let output_file = output_file.last().unwrap();

        inout::handle_output_redirection(command_args[0], &command_args[1..], Some(output_file));
    } else if let Some(input_idx) = parts.iter().position(|&x| x == "<") {
        let (command_args, input_file) = parts.split_at(input_idx);
        let input_file = input_file.last().unwrap();

        inout::handle_input_redirection(command_args[0], &command_args[1..], Some(input_file));
    }
}

fn handle_pipe(input: &str) {
    let parts: Vec<&str> = input.split("|").collect();
    let command1 = parts[0].trim();
    let command2 = parts[1].trim();

    let tokens1: Vec<&str> = command1.split_whitespace().collect();
    let tokens2: Vec<&str> = command2.split_whitespace().collect();

    inout::handle_pipe(tokens1[0], &tokens1[1..], tokens2[0], &tokens2[1..]);
}
