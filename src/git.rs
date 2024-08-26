use std::env;
use std::path::Path;
use std::process::{exit, Command};

pub struct GitModule {
    pub name: String,
    pub path: String,
    pub branch: String,
    pub submodules: Vec<GitModule>,
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

    pub fn print(&self) {
        println!(
            "name:\t {}\nbranch:\t {}\npath:\t {}\nsubmodules:",
            self.name, self.branch, self.path
        );
        for it in self.submodules.iter() {
            it.print();
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
    module.path = path;

    let res = Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .output()
        .expect("could not execute git");
    if !res.status.success() {
        println!("res {:?}", res.status.code());
        exit(0);
    }
    module.branch = String::from_utf8(res.stdout)
        .expect("could not parse git output")
        .trim()
        .to_string();

    return module;
}
