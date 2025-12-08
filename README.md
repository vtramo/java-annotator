# 🏷️ Java Annotator CLI
A simple CLI tool to automatically annotate Java source code files.

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
