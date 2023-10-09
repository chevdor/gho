# gho

A simple multi-platform cli utility written in Rust that opens your browser in your project’s repo on gitlab or github.

While a bash script like the following could work, that will not work on Windows:

    gho ()
    {
        ( set -e;
        git remote -v | grep push;
        remote=${1:-origin};
        echo "Using remote $remote";
        URL=$(git config remote.$remote.url | sed "s/git@\(.*\):\(.*\).git/https:\/\/\1\/\2/");
        echo "Opening $URL...";
        open $URL )
    }

`gho` on the other hand, should work fine (testers and feedback are welcome).

The url is fetched on your remotes.

## Install

    cargo install --locked --git https://github.com/chevdor/gh

## Usage

-   `gho` to simply open the first remote

-   `gho upstream` to open your `upstream` remote

<!-- -->

    Command line utility for the tera templating engine. You need to provide a template using the tera syntax as well as some data (various format are supported)

    Usage: gho [REMOTE]

    Arguments:
      [REMOTE]  Name of a remote

    Options:
      -h, --help     Print help
      -V, --version  Print version
