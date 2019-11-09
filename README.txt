changeme 0.1.0
Jack Brown <jack@brownjohnf.com>
Program description here

USAGE:
    changeme [FLAGS] [OPTIONS] [-- <option2>...] [SUBCOMMAND]

FLAGS:
    -n, --dry-run    Don't take any destructive actions
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose    Enable debug logging

OPTIONS:
    -o, --option1 <option1>    Optional string value
    -p, --path <path>          Path to some file or directory on the system

ARGS:
    <option2>...    Description for a list of strings

SUBCOMMANDS:
    get     Some sort of get operation, with flags
    help    Prints this message or the help of the given subcommand(s)
    list    Some sort of list operation
