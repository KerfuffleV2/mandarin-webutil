# Mandarin Web Utility

## Synopsis

Currently a very simple proof-of-concept tool to provide some readability functions for Simplified Chinese text input.

## Quickstart

**First**: You don't have to build it, you can just use it if you want. The current version should be published here: https://kerfufflev2.github.io/mandarin-webutil/

### Prerequisites

1. Ensure a recent version of the Rust toolchain is installed: https://rustup.rs/
2. To build for web, ensure the WASM target is installed: `rustup target add wasm32-unknown-unknown`
3. The `dioxus` utility is expected to be in your PATH: https://github.com/DioxusLabs/cli

### Building

If you have `make` installed you can run:

Command|Result
-|-
`make dev-web`|Compile for web and serve on port 8080.
`make build-web`|Compile for web in release mode.
`make dev-desktop`|Compiles and runs the app as a desktop program.
`make build-desktop`|Same as above, except it compiles in release mode.
`make clean`|Clean build/distribution directories.
`make deploy`|Deploys to `gh-pages` branch for GitHub Pages. (The branch must already exist.)

*Note*: Desktop build rules target Linux x86_64.

Otherwise refer to the [Makefile](./Makefile) for build commands.

## Limitations

The dictionary entries for words aren't in any particular order, which can result in unexpected pinyin output and useless definition tooltips.

For example, the common word **离** would produce `chi1` (archaic term for a mythical beast) rather than `li2` as expected.

The desktop version doesn't seem to correctly support clicking words to open the MDBG definition.

On Linux with Wayland, resizing the window seems to cause a crash. You can force it to run on X by setting the environment variable `GDK_BACKEND=x11`
