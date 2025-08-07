# IMPORTANT (to run) :- GO to https://webassembly.sh/ and run wapm upload then select the file that get downloaded via demo link to run this in browser!

### ✅ `README.md` for Local Rust Usage Only

````md
# rgit

A minimal Git-like CLI tool built in Rust.

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- Cargo (comes with Rust)

## 🚀 Run Locally

```bash
git clone https://github.com/Tech-with-anmol/rgit
cd rgit
cargo run -- <command>
````

Example:

```bash
cargo run -- init
cargo run -- add example.txt
cargo run -- commit -m "Initial commit"
cargo run -- status
```

> Note: `--` is required before passing CLI arguments to the binary during `cargo run`.

---

## 📁 Project Structure

```
rgit/
├── src/
│   └── main.rs
|   └── command.rs
|   └── Commands
|          └── mod.rs
|          └── ...
├── Cargo.toml
└── README.md
```

---

## 🛠 Commands

* `init` — Initialize a new repository
* `add <file>` — Stage a file
* `commit -m <msg>` — Commit with a message
* `status` — Show status of working directory
* `clone` - clone a repo
* `push` - push your files to github
* `write-tree ` - write to the tree
*  and more, listed by help command 

---

## 📜 License

[MIT](./LICENSE)


