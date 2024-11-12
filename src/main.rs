use git::read_git_module;

mod git;

fn main() {
    let mut git_module = read_git_module(String::from("root"), String::from("magic path"), false);

    git_module.print();
}
