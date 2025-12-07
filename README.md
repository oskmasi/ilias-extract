# 📂 Ilias Extract

**Ilias Extract** is a Rust command-line tool that 
extracts submissions zip files downloaded from **Ilias** and 
organizes them into a structured folder. 
It can also generate summary Markdown files for each team.

---

## ⚙ Features

| Feature                    | Description                                                   |
|----------------------------|---------------------------------------------------------------|
| 📥 **Extract Submissions** | Unzip and organize student submissions automatically.         |
| 📝 **Generate Summaries**  | Optionally generate a Markdown summary file for each student. |

---

## 📂 Folder Structure After Extraction

```
output/
├── alice123_bob345/
│   ├── submission_1.pdf
│   ├── submission_2.txt
│   └── summary.md    # if run with --summary
├── john425/
│   ├── submission_1.txt
│   └── summary.md
```

---

## 📦 Installation

Build from source using Cargo:

```bash
git clone https://github.com/oskmasi/ilias-extract.git
cd ilias-extract
cargo build --release
```

The executable will be available in:
`target/release/ilias-extract`

## 🚀 Usage

```bash
ilias-extract <INPUT_ZIP> <OUTPUT_DIR> [OPTIONS]
```

### Options

| Flag              | Description                                                 |
|-------------------|-------------------------------------------------------------|
| `-s, --summary`   | Generate a Markdown summary for each student.               |
| `-o, --overwrite` | Enable overwriting of existing files.                       |
| `--purge`         | Clear the target directory before extraction (⚠ careful!). |
| `-v, --verbose`   | Enable verbose output.                                      |

---

### Example Commands

| Command                                             | Description                                                 |
|-----------------------------------------------------|-------------------------------------------------------------|
| `ilias-extract submissions.zip ./output`            | Basic extraction.                                           |
| `ilias-extract submissions.zip ./output --summary`  | Generate summaries for each team.                           |
| `ilias-extract submissions.zip ./output -s --purge` | Clears output folder and generates summaries for each team. |


## 🔧 Dependencies

- [`clap`](https://crates.io/crates/clap) – Command-line argument parsing
- [`anyhow`](https://crates.io/crates/anyhow) – Error handling
- [`zip`](https://crates.io/crates/zip) – Zip file handling
- [`regex`](https://crates.io/crates/regex) – Regex pattern matching
- [`calamine`](https://crates.io/crates/calamine) – Excel file reading
- [`tempfile`](https://crates.io/crates/tempfile) – Temporary file handling
- [`chrono`](https://crates.io/crates/chrono) – Date and time utilities

Contributions, issues, and feature requests are welcome!
Feel free to fork the repo and submit a pull request.
