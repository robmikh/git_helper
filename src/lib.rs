use std::process::Command;

pub struct GitHelper {
    directory: Option<String>,
}

impl GitHelper {
    pub fn create_from_current_dir() -> GitHelper {
        GitHelper {
            directory: None,
        }
    }

    pub fn create_from_dir(directory: &str) -> GitHelper {
        GitHelper {
            directory: Some(directory.to_string()),
        }
    }

    pub fn rev_list(&self, commit: &str, count: i32) -> Vec<String> {
        let output = self.build_git_command(&["rev-list", commit, "--first-parent", "--max-count", &count.to_string()])
            .output()
            .expect("Failed to run git rev-list command!");

        let commits = String::from_utf8(output.stdout).unwrap().split('\n')
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();

        commits
    }

    pub fn name_rev(&self, commit: &str, pattern: &str) -> Option<String> {
        let output = self.build_git_command(&["name-rev", "--ref", pattern, "--name-only", "--no-undefined", commit])
            .output()
            .expect("Failed to run git name-rev command!");

        let mut result: Option<String> = None;
        if output.status.success() {
            let name = String::from_utf8(output.stdout).unwrap();
            let name = name.trim().split('~').next().unwrap();
            result = Some(name.to_string());
        }

        result
    }

    pub fn find_ancestor_with_name(&self, pattern: &str, page_size: i32) -> Option<String> {
        self.find_ancestor_of_commit_with_name("HEAD", pattern, page_size)
    }

    pub fn find_ancestor_of_commit_with_name(&self, commit: &str, pattern: &str, page_size: i32) -> Option<String> {
        let mut result: Option<String> = None;

        let mut current_commit = String::from(commit);
        'outer: loop {
            let current_page = self.rev_list(&current_commit, page_size);

            if current_page.len() == 0 {
                break;
            }

            for commit in &current_page {
                if let Some(name) = self.name_rev(&commit, pattern) {
                    result = Some(name);
                    break 'outer;
                }
            }

            // If there was only one commit that came back, there aren't any more that we can get. 
            if current_page.len() == 1 {
                break;
            }

            current_commit = current_page.last().unwrap().to_string();
        }

        result
    }

    fn build_git_command<I, S>(&self, args: I) -> Command
        where I: IntoIterator<Item=S>, S: AsRef<std::ffi::OsStr>
    {
        let mut command = Command::new("git");
        command.args(args);

        if let Some(directory) = &self.directory {
            command.current_dir(directory);
        }

        command
    }
}