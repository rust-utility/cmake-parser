use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let Some(gen) = args() else {
        eprintln!("Usage: gen <command_type> <command> [comment]");
        return ExitCode::FAILURE;
    };

    match gen.run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("{err}");
            ExitCode::FAILURE
        }
    }
}

struct Gen {
    command_type: String,
    command: String,
    comment: Option<String>,
    command_name: Option<String>,
}

impl Gen {
    fn run(&self) -> Result<(), std::io::Error> {
        let Self {
            command_type,
            command,
            comment,
            command_name,
        } = self;

        let command_safe = {
            use check_keyword::CheckKeyword as _;
            command.as_str().into_safe()
        };

        {
            let fixture_path = Path::new("fixture")
                .join("commands")
                .join(command_type)
                .join(command);

            if fixture_path.exists() {
                eprintln!(
                    "Skipping existing fixture {}... ok",
                    fixture_path.to_string_lossy()
                );
            } else {
                self.write_file(&fixture_path, format!("{command}(name)\n"))?;
            }
        }

        let (command_name, command_type_name) = {
            use inflections::Inflect as _;
            (
                command_name.clone().unwrap_or_else(|| {
                    command
                        .to_pascal_case()
                        .replace("Ctest", "CTest")
                        .replace("Cmake", "CMake")
                }),
                if command_type != "ctest" {
                    command_type.to_pascal_case()
                } else {
                    "CTest".into()
                },
            )
        };

        {
            let command_mod_rs_path = Path::new("lib")
                .join("src")
                .join("doc")
                .join("command")
                .join("mod.rs");
            let content = std::fs::read_to_string(&command_mod_rs_path)?;

            let mut lines = content.lines().map(str::to_string).collect::<Vec<_>>();

            let declaration =
                format!("    {command_name}(Box<{command_type}::{command_name}<'t>>),");

            if let Some(declaration_pos) = lines.iter().position(|l| l == &declaration) {
                if let Some(new_comment) = self.comment.as_deref() {
                    eprint!("Command declaration is found, updating comment...");
                    if declaration_pos != 0 {
                        if let Some(comment) = lines
                            .get_mut(declaration_pos - 1)
                            .filter(|line| line.trim_start().starts_with("///"))
                        {
                            *comment = format!("    /// {new_comment}")
                        }
                    }
                    eprintln!(" ok");
                } else {
                    eprintln!("Command declaration is found, skipping... ok");
                }
            } else if let Some(closing_bracket_pos) = lines.iter().position(|l| l == "}") {
                eprint!("Command declaration is not found, inserting...");
                lines.insert(closing_bracket_pos, declaration);
                let comment = format!("    /// {}", comment.as_deref().unwrap_or_default());
                lines.insert(closing_bracket_pos, comment);
                eprintln!(" ok");
            } else {
                eprintln!(
                    "Could not update {}!",
                    command_mod_rs_path.to_string_lossy()
                );
                return Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "failed to update Command struct",
                ));
            }
            self.write_if_changed(lines, content, &command_mod_rs_path)?;
        }

        {
            let command_type_mod_rs_path = Path::new("lib")
                .join("src")
                .join("doc")
                .join("command")
                .join(command_type)
                .join("mod.rs");
            let content = std::fs::read_to_string(&command_type_mod_rs_path)?;

            let mut lines = content.lines().map(str::to_string).collect::<Vec<_>>();

            let declaration_mod = format!("mod {command_safe};");

            if lines.contains(&declaration_mod) {
                eprintln!("Module declaration is found, skipping... ok");
            } else {
                eprint!("Module declaration not found, adding...");
                let empty_line_pos = lines.iter().position(|s| s.is_empty()).unwrap_or_default();
                lines.insert(empty_line_pos, declaration_mod);
                eprintln!(" ok");
            }

            let declaration_pub_use = format!("pub use {command_safe}::{command_name};");

            if lines.contains(&declaration_pub_use) {
                eprintln!("Public use declaration is found, skipping... ok");
            } else {
                eprint!("Public use declaration not found, adding...");
                lines.push(declaration_pub_use);
                eprintln!(" ok");
            }

            self.write_if_changed(lines, content, &command_type_mod_rs_path)?;
        }

        {
            let command_rs_path = Path::new("lib")
                .join("src")
                .join("doc")
                .join("command")
                .join(command_type)
                .join(format!("{command}.rs"));
            if command_rs_path.exists() {
                eprintln!(
                    "Skipping existing command mod {}... ok",
                    command_rs_path.to_string_lossy()
                );
            } else {
                eprint!("Writing {}...", command_rs_path.to_string_lossy());
                let comment = comment.as_deref().unwrap_or_default();
                let content = format!(
                    include_str!("command_rs.template"),
                    command = command,
                    command_name = command_name,
                    command_type = command_type,
                    command_type_name = command_type_name,
                    command_safe = command_safe,
                    comment = comment,
                );
                std::fs::write(command_rs_path, content)?;
                eprintln!(" ok");
            }
        }

        {
            let doc_mod_rs_path = Path::new("lib").join("src").join("doc").join("mod.rs");
            let content = std::fs::read_to_string(&doc_mod_rs_path)?;

            let mut lines = content.lines().map(str::to_string).collect::<Vec<_>>();

            let to_command = format!("to_command(tokens, Command::{command_name})");
            let match_case = format!("                b\"{command}\" => {to_command},");

            if lines.iter().any(|l| l.contains(&to_command)) {
                eprintln!("Match is found, skipping... ok");
            } else {
                eprint!("Match is not found, adding...");
                if let Some(unknown_pos) = lines
                    .iter()
                    .position(|l| l.trim_start().starts_with("unknown =>"))
                {
                    lines.insert(unknown_pos, match_case);
                    eprintln!(" ok");
                } else {
                    eprintln!(" fail: `unknown =>` not found");
                }
            }

            self.write_if_changed(lines, content, &doc_mod_rs_path)?;
        }

        {
            let readme_md_path = Path::new("README.md");
            let content = std::fs::read_to_string(readme_md_path)?;
            let mut lines = content.lines().map(str::to_string).collect::<Vec<_>>();

            let checkbox_empty = format!("- [ ] {command}");

            if let Some(checkbox_line) = lines.iter_mut().find(|l| l == &&checkbox_empty) {
                eprint!("Updating checkbox...");
                checkbox_line.replace_range(3..4, "x");
                eprintln!(" ok");
            }

            let count_checked = lines.iter().filter(|l| l.starts_with("- [x]")).count();
            let count_unchecked = lines.iter().filter(|l| l.starts_with("- [ ]")).count();
            let count_total = count_checked + count_unchecked;

            let implemented = format!("Implemented: {count_checked} of {count_total}.");
            if let Some(implemented_line) = lines
                .iter_mut()
                .find(|l| l.starts_with("Implemented: ") && l != &&implemented)
            {
                eprint!("Updating implemented count...");
                *implemented_line = implemented;
                eprintln!(" ok");
            }

            self.write_if_changed(lines, content, readme_md_path)?;
        }
        Ok(())
    }

    fn write_if_changed(
        &self,
        lines: Vec<String>,
        content: String,
        path: &Path,
    ) -> Result<(), std::io::Error> {
        let mut result_content = lines.join("\n");

        if content.ends_with('\n') && !result_content.ends_with('\n') {
            result_content.push('\n');
        }

        if content != result_content {
            self.write_file(path, result_content)?;
        }
        Ok(())
    }

    fn write_file(&self, p: &Path, content: String) -> Result<(), std::io::Error> {
        eprint!("Writing {}...", p.to_string_lossy());
        std::fs::write(p, content)?;
        eprintln!(" ok");
        Ok(())
    }
}

fn args() -> Option<Gen> {
    let mut args = std::env::args().skip(1);
    let command_type = args.next()?;
    let command = args.next()?;
    let comment = args.next();
    let command_name = args.next();
    Some(Gen {
        command_type,
        command,
        comment,
        command_name,
    })
}
