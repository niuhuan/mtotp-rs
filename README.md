mTotp
======

mTotp is mean **M**y **Totp**, a simple TOTP manager for command line.

I don't like using my phone for two factor authentication so I made this tool.

## 📦 Installation

```bash
cargo install mtotp
```

## 📖 Usage

### Print totp codes

```shell
mtotp list
```

```
┍ -------------------- ┯ ---------- ┑
| label                |       code |
| -------------------- ┿ ---------- |
| GithubN              |     123456 |
| GithubJ              |     000765 |
└ -------------------- ┴ ---------- ┘
```

### Print help

```bash
mtotp --help
```

```
Usage: mtotp <COMMAND>

Commands:
  list    List registered totp and codes
  add     Add new totp
  remove  Remove totp
  rename  Rename a totp label
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

## 🔖 Tips

The database location:
 - windows: `%HOME%\AppData\Roaming\mtotp\`
 - linux: `$HOME/.mtotp/`
 - macos: `$HOME/Library/Application Support/mtotp/`

## 📕 License

Reference `LICENSE` File

