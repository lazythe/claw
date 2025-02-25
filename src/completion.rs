use rustyline::completion::{Completer, Pair};
use rustyline::Context;
use std::path::PathBuf;
use std::fs;
use std::env;

pub struct FileCompleter;

impl FileCompleter {
    pub fn new() -> Self {
        FileCompleter
    }

    fn get_completions(&self, line: &str, pos: usize) -> Vec<Pair> {
        let (prefix, word_to_complete) = self.extract_word(line, pos);
        let is_cd_command = prefix.trim_end().ends_with("cd");
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        
        if word_to_complete == ".." {
            if let Some(_parent) = current_dir.parent() {
                return vec![Pair {
                    display: "../".to_string(),
                    replacement: "../".to_string(),
                }];
            }
        }
        
        let (search_dir, file_prefix) = if word_to_complete.contains('/') {
            let path = if word_to_complete.starts_with('/') {
                PathBuf::from(word_to_complete)
            } else {
                current_dir.join(word_to_complete)
            };
            
            let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_string();
            let dir_path = if word_to_complete.ends_with('/') {
                path
            } else {
                let mut path_clone = path;
                path_clone.pop();
                path_clone
            };

            (dir_path, file_name)
        } else {
            (current_dir, word_to_complete.to_string())
        };

        let mut entries = Vec::new();
        if let Ok(dir_entries) = fs::read_dir(&search_dir) {
            let mut dir_entries: Vec<_> = dir_entries
                .filter_map(Result::ok)
                .filter(|entry| {
                    let name = entry.file_name().to_string_lossy().to_lowercase();
                    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                    
                    if is_cd_command && !is_dir {
                        return false;
                    }
                    
                    prefix.ends_with(' ') || file_prefix.is_empty() || name.starts_with(&file_prefix.to_lowercase())
                })
                .collect();
            
            dir_entries.sort_by(|a, b| {
                let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                
                if a_is_dir == b_is_dir {
                    a.file_name().cmp(&b.file_name())
                } else {
                    b_is_dir.cmp(&a_is_dir)
                }
            });
            
            for entry in dir_entries {
                let name = entry.file_name().to_string_lossy().into_owned();
                let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                
                let display = if is_dir { format!("{}/", name) } else { name.clone() };
                
                let replacement = if word_to_complete.contains('/') {
                    let mut new_path = PathBuf::from(word_to_complete);
                    if word_to_complete.ends_with('/') {
                        new_path.push(&name);
                    } else {
                        new_path.pop();
                        new_path.push(&name);
                    }
                    if is_dir {
                        format!("{}/", new_path.to_string_lossy())
                    } else {
                        new_path.to_string_lossy().into_owned()
                    }
                } else {
                    if is_dir {
                        format!("{}/", name)
                    } else {
                        name
                    }
                };
                
                entries.push(Pair { display, replacement });
            }
        }

        entries
    }

    fn extract_word<'a>(&self, line: &'a str, pos: usize) -> (String, &'a str) {
        let line_until_pos = &line[..pos];
        if let Some(last_space) = line_until_pos.rfind(' ') {
            (line_until_pos[..=last_space].to_string(), &line_until_pos[last_space + 1..])
        } else {
            (String::new(), line_until_pos)
        }
    }
}

impl Completer for FileCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (prefix, _) = self.extract_word(line, pos);
        let start_pos = prefix.len();
        
        let completions = self.get_completions(line, pos);
        Ok((start_pos, completions))
    }
}
