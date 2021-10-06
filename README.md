**This branch is unmaintained. For the current version, see [the `main` branch](https://github.com/fenhl/webloc-cli/tree/main).**

`webloc` is a command-line utility for creating and reading .webloc files. .webloc is the file format macOS uses to store URLs. The script is a simple wrapper around [the webloc gem](https://github.com/peterc/webloc).

# Installation

1. `gem install docopt webloc`
2. Clone the repository
3. Make a symlink called `webloc` in your `PATH` which points at `webloc.rb`

# Usage

Inspect .webloc files with `webloc read`, or create them with `webloc save`.

## Read

The syntax is `webloc read <filename>`, and will print the URL to stdout. Due to how .webloc files work, piping a .webloc file into the script and using `-` as the filename parameter is not supported.

## Save

The syntax is `webloc save <filename> [<url>]`. If the url parameter is omitted, the script will attempt to read a URL from stdin. Note that URLs are not validated, allowing you to create broken .webloc files which may behave oddly.
