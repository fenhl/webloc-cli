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

1. (Skip this step if you're not on Windows.) If you're on Windows, you'll first need to download and install [Visual Studio](https://visualstudio.microsoft.com/vs/) (the Community edition should work). On the “Workloads” screen of the installer, make sure “Desktop development with C++” is selected. (Note that [Visual Studio Code](https://code.visualstudio.com/) is not the same thing as Visual Studio. You need VS, not VS Code.)
2. Install Rust:
    * On Windows, download and run [rustup-init.exe](https://win.rustup.rs/) and follow its instructions.
    * On other platforms, please see [the Rust website](https://www.rust-lang.org/tools/install) for instructions.
3. Open a command line:
    * On Windows, right-click the start button, then click “Windows PowerShell” or “Command Prompt”.
    * On other platforms, look for an app named “Terminal” or similar.
4. In the command line, run the following command. Depending on your computer, this may take a while.

    ```
    cargo install --git=https://github.com/fenhl/webloc-cli --branch=main
    ```

# Usage

Inspect .webloc files with `webloc read`, or create them with `webloc save`.

## Read

The syntax is `webloc read [<filename>]`, and will print the URL to stdout. If the filename parameter is omitted, the script will attempt to read a webloc file from stdin.

## Save

The syntax is `webloc save [<filename>] [<url>]`. If the url parameter is omitted, the script will attempt to read a URL from stdin. If the filename parameter is omitted, the script will write the webloc file to stdout.

The `--xml` (or `-x`) flag can be passed to make `webloc` output a human-readable XML webloc file instead of using the more compact binary format. Both formats are equally supported by macOS (and `webloc read`).
