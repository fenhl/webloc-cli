`webloc` is a command-line utility for creating and reading .webloc files. .webloc is the file format macOS uses to store URLs.

# Installation

## macOS

1. [Download `webloc`](https://github.com/fenhl/webloc-cli/releases/latest/download/webloc)
2. Mark `webloc` as executable (`chmod +x Downloads/webloc`) and move it to somewhere in your `PATH`.
3. Try running `webloc -h`. You will get a permission error that `webloc` is from an unidentified developer.
4. Open System Preferences → Security & Privacy → General and click the button to allow `webloc` to run.
5. Now `webloc -h` will show another warning, after which it should output the help text.

## From source

If you're not on macOS or would prefer to build the tool from source, follow these instructions:

1. Install Rust:
    * On Windows, download and run [rustup-init.exe](https://win.rustup.rs/) and follow its instructions. If asked to install Visual C++ prerequisites, use the “Quick install via the Visual Studio Community installer” option. You can uncheck the option to launch Visual Studio when done.
    * On other platforms, please see [the Rust website](https://www.rust-lang.org/tools/install) for instructions.
2. Open a command line:
    * On Windows, right-click the start button, then click “Terminal”, “Windows PowerShell”, or “Command Prompt”.
    * On other platforms, look for an app named “Terminal” or similar.
3. In the command line, run the following command. Depending on your computer, this may take a while.

    ```
    cargo install --git=https://github.com/fenhl/webloc-cli --branch=main webloc-cli
    ```

# Usage

Inspect .webloc files with `webloc read`, or create them with `webloc save`.

## Read

The syntax is `webloc read [<path>]`, and will print the URL to stdout. If the path parameter is omitted, the script will attempt to read a webloc file from stdin.

## Save

The syntax is `webloc save [<path>] [<url>]`. If the url parameter is omitted, the script will attempt to read a URL from stdin. If the path parameter is omitted, the script will write the webloc file to stdout.

The `--xml` (or `-x`) flag can be passed to make `webloc save` output a human-readable XML webloc file instead of using the more compact binary format. Both formats are equally supported by macOS (and `webloc read`).
