# git-helper

Tiny Git helper tool written in Rust for some daily tasks.

## Usage

    Usage: git-helper [OPTIONS]
    
    Options:
    -p, --path <PATH>        The path to git repo [default: ./]
    -c, --command <COMMAND>  The command to execute [default: list] [possible values: list]
    -f, --filter <FILTER>    Filter branches. Use '!' at the beginning for unequal [default: ]
    -r, --recursive          Crawl all submodules
    -h, --help               Print help
    -V, --version            Print version

