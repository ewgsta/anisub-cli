# Anisub CLI
> **Read this in other languages:** [Türkçe](README.tr.md)

CLI tool for the Turkish anime subtitles platform anisub.co.

## Installation
Make sure Rust and Cargo are installed.
```bash
cargo install anisub-cli
```

or install via AUR:
```bash
yay -S anisub-cli
```

## Usage

### Login (Optional)

You can log in to download using your personal token.
[Click here to get a token.](https://anisub.co/ayarlar#api)

```bash
anisub-cli login
```

### Search and Download

You can search by anime or subtitle name.
```bash
anisub-cli search "bleach"
```

or use the `-o` parameter to download to a specific folder:
```bash
anisub-cli search "bleach" -o ~/Downloads/Subtitles
```

### Shortcuts (In Search Screen)
* **Up/Down Arrow:** Navigate between results
* **Enter:** Download selected subtitle
* **Q / ESC:** Exit
