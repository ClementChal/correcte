# Correcte

Correcte is a Rust desktop utility for correcting clipboard text locally. A keyboard shortcut opens a single editor with the current clipboard text. You can edit it, run a correction, and copy the result back into another application.

## Current status

The Slint prototype has been validated on Debian 13, GNOME on X11, and x86-64. Correction is currently simulated.

- The editor receives keyboard focus on opening. It becomes read-only during a correction, then the result replaces the text and editing resumes.
- Copy leaves the window open. Escape and the window close button discard the session. The next opening loads the current clipboard contents.
- Copying and pasting after closing has been verified on the reference machine. Release startup performance remains to be measured.

## Build and try the prototype

On Debian, install a Rust toolchain and the build dependencies:

```sh
sudo apt install build-essential pkg-config libfontconfig-dev
cargo build --locked
```

Copy some text and run `./target/debug/correcte`. To open it from any application, add a custom shortcut in GNOME Keyboard settings and set the command to the absolute path of that executable.

Run the Rust tests with `cargo test --locked`.

## Planned V1

- **Startup** · target 300 ms at the 95th percentile from triggering the shortcut to typing in the editor. If cold starts pass and the copied result remains available after exit, Correcte exits on close. Otherwise, it stays running with the window hidden.
- **Local correction** · the Debian package will include a CPU build of `llama-server`. Initial setup will download a GGUF model. The default prompt will correct spelling, grammar, and punctuation offline while preserving the language, tone, and line breaks. Sentences that would require reordering words stay unchanged.
- **Settings** · `~/.config/correcte/config.toml` on Debian by default. The model path and system prompt will be configurable.
- **Diagnostics** · local logs capped at 3 MiB for timings, status, and errors. Logs will exclude the clipboard text, prompt, and correction result.

Flatpak is the preferred option to evaluate for broader Linux distribution after the V1.
