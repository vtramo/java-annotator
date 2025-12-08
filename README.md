[crates-url]: https://crates.io/crates/annotator
[license-badge]: https://img.shields.io/crates/l/annotator.svg
[crates-badge]: https://img.shields.io/crates/v/annotator.svg

# 🏷️ Java Annotator CLI [![Crates.io][crates-badge]][crates-url] ![License][license-badge]
A simple CLI tool to automatically annotate Java types (class, interface, enum, inner class, or inner interface)
with a set of specified Java annotations.

# 💡 What It Does
This tool adds the specified Java annotations to all encountered Java types (class, interface, enum, inner class, or inner interface)
that do not already possess that particular Java annotation.
- The tool does not verify whether the specified annotation (e.g., @MyCustomTag) is actually defined or exists in the project's classpath.
- Annotations are only added to the type declaration itself; methods, fields, and parameters are currently ignored.
- The tool does not check if the annotation's usage is valid on the specific type.
- The tool is guaranteed to function correctly only with simple annotations that do not include arguments,
  values, or parentheses. Complex annotations like are not currently supported and may lead to incorrect parsing or modifications.
## Usage
Process Java files in a path (file or directory) with specified annotations.
```bash
cargo run -- <PATH> -a <ANNOTATION>
```
### Example
```bash
cargo run -- src/java -a @Generated -a @CustomAnnotation 

src/java/java/a/b/C.java C
src/java/a/B.java B
src/java/A.java A
```
The output exclusively lists the paths of the Java source files that were actually modified, followed by the name of
the Java types (class, interface, enum, inner class, or inner interface) that received the annotation within that file.
