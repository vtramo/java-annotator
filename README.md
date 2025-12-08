[crates-url]: https://crates.io/crates/annotator
[license-badge]: https://img.shields.io/crates/l/annotator.svg
[crates-badge]: https://img.shields.io/crates/v/annotator.svg

# 🏷️ Java Annotator CLI [![Crates.io][crates-badge]][crates-url] ![License][license-badge]
A simple CLI tool to automatically annotate Java source code files.

# 💡 What It Does
This tool adds the specified annotations to all encountered Java types (classes, interfaces, enums, etc.) that do not already possess that particular annotation.

## Usage
Process Java files in a path (file or directory) with specified annotations.
```bash
cargo run -- <PATH> -a <ANNOTATION>
```
### Example
```bash
cargo run -- src/java -a @Override -a @CustomTag

src/java/java/a/b/C.java C
src/java/a/B.java B
src/java/A.java A
```
The output lists only the paths of the modified Java files and the type (class, interface, enum, etc.) that was annotated within that file.
