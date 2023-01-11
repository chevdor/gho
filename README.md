# gh

A simple multi-platform cli utility written in Rust that opens your browser in your project’s repo on gitlab or github.

While a bash script like the following could work, that will not work on Windows:

    gh ()
    {
        ( set -e;
        git remote -v | grep push;
        remote=${1:-origin};
        echo "Using remote $remote";
        URL=$(git config remote.$remote.url | sed "s/git@\(.*\):\(.*\).git/https:\/\/\1\/\2/");
        echo "Opening $URL...";
        open $URL )
    }

`gh` on the other hand, should work fine (testers and feedback are welcome).

The url is fetched on your remotes.

## Install

    cargo install --locked --git https://github.com/chevdor/gh --tag v0.2.0

## Usage

-   `gh` to simply open the first remote

-   `gh upstream` to open your `upstream` remote

<!-- -->

    Command line utility for the tera templating engine. You need to provide a template using the tera syntax as well as some data (various format are supported)

    Usage: gh [REMOTE]

    Arguments:
      [REMOTE]  Name of a remote

    Options:
      -h, --help     Print help information
      -V, --version  Print version information
