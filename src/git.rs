use regex::Regex;
use std::env;
use std::fs::read_to_string;
use std::path::Path;
use std::process::exit;
use std::process::Command;

pub struct GitModule {
    pub name: String,
    pub path: String,
    pub branch: String,
    pub submodules: Vec<GitModule>,
}

struct GitSubmodule {
    pub name: String,
    pub path: String,
}

fn filter_module(git_module: &GitModule, filter_branch: &String) -> bool {
    if filter_branch.is_empty() {
        return true;
    }
    if filter_branch.starts_with('!') {
        let str = filter_branch[1..filter_branch.len()].to_string();
        return str != git_module.branch;
    }

    return filter_branch == &git_module.branch;
}

impl GitModule {
    pub fn new() -> Self {
        GitModule {
            name: String::from("root"),
            path: String::from("./"),
            branch: String::from("main"),
            submodules: Vec::new(),
        }
    }

    pub fn print(&self, filter_branch: &String) {
        let res = filter_module(self, filter_branch);
        if res {
            if self.name != "root" {
                println!("---");
            }
            println!(
                "name:\t {}\nbranch:\t {}\npath:\t {}",
                self.name, self.branch, self.path
            );
        }
        for it in self.submodules.iter() {
            it.print(filter_branch);
        }
    }
}

pub fn read_git_module(name: String, path: String, recursive: bool) -> GitModule {
    let mut module = GitModule::new();

    // change working dir
    {
        let path = Path::new(path.as_str());
        assert!(env::set_current_dir(&path).is_ok()); // TODO: check result and fail if does not work
    }

    module.name = name;
    module.path = path.clone();

    let res = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("could not execute git");
    if !res.status.success() {
        println!("git failed: {}", res.status.code().unwrap());
        exit(1);
    }
    module.branch = String::from_utf8(res.stdout)
        .expect("could not parse git output")
        .trim()
        .to_string();
    if module.branch.is_empty() {
        module.branch = String::from("detached");
    }

    // submodules
    if recursive {
        let mut gitmodules_path = path.clone();
        gitmodules_path.push_str("/.gitmodules");
        if Path::new(gitmodules_path.as_str()).exists() {
            let submodule_reg = Regex::new("\\[submodule \"(.+)\"\\]").unwrap();
            let path_reg = Regex::new("path = (.+)").unwrap();
            let mut found_submodules: Vec<GitSubmodule> = Vec::new();

            for line in read_to_string(&gitmodules_path).unwrap().lines() {
                let submodule = submodule_reg.captures(line);

                if submodule.is_some() {
                    found_submodules.push(GitSubmodule {
                        name: String::from(submodule.unwrap().get(1).unwrap().as_str()),
                        path: String::from(""),
                    });
                } else {
                    let path_res = path_reg.captures(line);

                    if path_res.is_some() {
                        found_submodules.last_mut().unwrap().path =
                            String::from(path_res.unwrap().get(1).unwrap().as_str());
                    }
                }
            }

            for it in found_submodules {
                if !it.name.is_empty() && !it.path.is_empty() {
                    let mut absolue_path = path.clone();
                    absolue_path.push('/');
                    absolue_path.push_str(&it.path);
                    module
                        .submodules
                        .push(read_git_module(it.name, absolue_path, recursive));
                }
            }
        }
    }

    return module;
}
