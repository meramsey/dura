use std::{fs, path, process::Command};

/// A test utility to make our tests more readable
pub struct GitRepo {
    // implements Drop to delete the directory
    pub dir: path::PathBuf,

    // Source of entropy for change_file
    counter: u32,
}

impl GitRepo {
    pub fn new(dir: path::PathBuf) -> Self {
        Self { dir, counter: 0 }
    }

    pub fn git(&self, args: &[&str]) -> Option<String> {
        println!("$ git {}", args.join(" "));
        let git_dir = self.dir.as_path().join(path::Path::new(".git"));

        let child_proc = Command::new("git")
            .args(
                [
                    &[
                        "--git-dir",
                        git_dir.to_str().unwrap(),
                        "--work-tree",
                        self.dir.as_path().to_str().unwrap(),
                    ],
                    args,
                ]
                .concat(),
            )
            .output();

        if let Ok(output) = child_proc {
            if !output.status.success() {
                // This cleans up test development by causing us to fail earlier
                return None;
            }
            let text = String::from_utf8(output.stdout).unwrap();
            if !text.is_empty() {
                println!("{}", text);
            }
            let err = String::from_utf8(output.stderr).unwrap();
            if !err.is_empty() {
                println!("{}", err);
            }
            Some(text)
        } else {
            None
        }
    }

    pub fn init(&self) {
        fs::create_dir_all(self.dir.as_path()).unwrap();
        let _ = self.git(&["init"]);
        let _ = self.git(&["checkout", "-b", "master"]);
    }

    pub fn commit_all(&self) {
        self.git(&["add", "."]);
        self.git(&["status"]);
        self.git(&["commit", "-m", "test"]);
    }

    pub fn write_file(&self, path: &str) {
        let content = "initial rev";
        let path_obj = self.dir.as_path().join(path);
        println!("$ echo '{}' > {}", content, path);
        fs::write(path_obj, content).unwrap();
    }

    /// Every time this is called it overwrites the file with **different** contents.
    pub fn change_file(&mut self, path: &str) {
        self.counter += 1;
        let content = format!("change {}", self.counter);
        println!("$ echo '{}' > {}", content, path);
        let path_obj = self.dir.as_path().join(path);
        fs::write(path_obj, content).unwrap();
    }
}
