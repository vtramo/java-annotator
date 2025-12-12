use std::borrow::Cow;
use crate::declaration::{build_java_declaration, JavaTypeDeclaration};
use crate::io::JavaFile;
use anyhow::anyhow;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use tree_sitter::Tree;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JavaAnnotation {
    annotation_name: String,
    fully_qualified_name: String,
}

impl Deref for JavaAnnotation {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.annotation_name
    }
}

impl JavaAnnotation {
    pub fn new(annotation: &str) -> anyhow::Result<JavaAnnotation> {
        if !annotation.starts_with('@') {
            return Err(anyhow!("Annotation '{}' must start with '@'", annotation));
        }

        let splitted_annotation = annotation[1..].split(".");
        let annotation_name = splitted_annotation
            .last()
            .ok_or_else(|| anyhow!("Annotation '{}' is invalid", annotation))?;

        Ok(
            JavaAnnotation {
                annotation_name: "@".to_owned() + annotation_name,
                fully_qualified_name: annotation[1..].to_string(),
            }
        )
    }

    pub fn fully_qualified_name(&self) -> &str {
        &self.fully_qualified_name
    }

    pub fn import(&self) -> String {
        format!(
            "import {};",
            self.fully_qualified_name
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JavaFileEdit {
    java_file: JavaFile,
    modified_types: HashSet<String>,
}

impl JavaFileEdit {
    pub fn is_modified(&self) -> bool {
        !self.modified_types.is_empty()
    }
}

impl Display for JavaFileEdit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let modified_types: String = self
            .modified_types
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>()
            .join(", ");

        f.write_str(&format!(
            "{} {}",
            self.java_file.path().to_owned(),
            modified_types
        ))
    }
}

pub fn annotate_java_files(
    java_files: &Vec<JavaFile>,
    annotations: &Vec<JavaAnnotation>,
) -> Vec<JavaFileEdit> {
    java_files
        .iter()
        .map(|java_file| annotate_file(java_file, annotations))
        .collect()
}

pub fn annotate_file(java_file: &JavaFile, annotations: &Vec<JavaAnnotation>) -> JavaFileEdit {
    let path = java_file.path();
    let src = std::fs::read_to_string(path).unwrap();
    let (new_src, modified_types) = annotate(&src, annotations);
    std::fs::write(path, new_src).unwrap();
    JavaFileEdit {
        java_file: java_file.clone(),
        modified_types,
    }
}

pub fn annotate(src: &str, annotations: &Vec<JavaAnnotation>) -> (String, HashSet<String>) {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_java::LANGUAGE.into())
        .expect("Failed to load Java language");

    let mut is_tree_modified = true;
    let mut text = src.to_string();
    let mut modified_types = HashSet::new();
    while is_tree_modified {
        is_tree_modified = false;

        let tree = parser.parse(text.to_string(), None).unwrap();
        let root_node = tree.root_node();

        let mut childs = Vec::new();
        childs.push(root_node);
        while !childs.is_empty() {
            let node = childs.pop().unwrap();

            if is_type(&node) {
                let declaration = build_java_declaration(&node, &text);
                for annotation in annotations {
                    if !declaration.contains_marker_annotation(annotation) {
                        text = add_annotation(&text, annotation, &declaration);
                        text = add_import_if_not_exists(&text, annotation, &tree).to_string();
                        modified_types.insert(declaration.name().to_string());
                        childs.clear();
                        is_tree_modified = true;
                        break;
                    }
                }
            }

            for child in 0..node.child_count() {
                childs.push(node.child(child).unwrap());
            }
        }
    }

    (text, modified_types)
}

fn add_import_if_not_exists<'a>(text: &'a str, annotation: &JavaAnnotation, tree: &Tree) -> Cow<'a, str> {
    let imports = find_all_imports(tree, text);
    let annotation_import = annotation.import();

    if !imports.contains(&annotation_import) {
        return match find_start_byte_imports(tree) {
            None => {
                let last_byte_package_declaration = find_last_byte_package_declaration(tree);
                Cow::Owned(
                    text[..last_byte_package_declaration].to_string() +
                    "\n\n" + &annotation_import +
                    "\n" + &text[last_byte_package_declaration..]
                )
            },
            Some(start_byte) =>
                Cow::Owned(
                    text[..start_byte].to_string() +
                    annotation_import.as_str() +
                    "\n" + &text[start_byte..]
                ),
        }
    }

    Cow::Borrowed(text)
}

fn find_last_byte_package_declaration(tree: &Tree) -> usize {
    let root_node = tree.root_node();
    root_node.children(&mut root_node.walk())
        .find(|node| node.kind() == "package_declaration")
        .map(|node| node.end_byte())
        .unwrap_or(0)
}

fn find_start_byte_imports(tree: &Tree) -> Option<usize> {
    let root_node = tree.root_node();
    root_node.children(&mut root_node.walk())
        .find(|node| node.kind() == "import_declaration")
        .map(|node| node.start_byte())
}

fn find_all_imports(tree: &Tree, src: &str) -> Vec<String> {
    let root_node = tree.root_node();
    root_node.children(&mut root_node.walk())
        .filter(|node| node.kind() == "import_declaration")
        .map(
            |node| node
                .utf8_text(src.as_bytes())
                .unwrap()
                .to_string()
        )
        .collect()
}

fn is_type(node: &tree_sitter::Node) -> bool {
    node.kind() == "class_declaration" || node.kind() == "interface_declaration" || node.kind() == "enum_declaration"
}

fn add_annotation(src: &str, annotation: &str, declaration: &JavaTypeDeclaration) -> String {
    src[..declaration.start_byte()].to_string()
        + annotation
        + "\n"
        + " ".repeat(declaration.tot_whitespaces()).as_str()
        + &src[declaration.start_byte()..]
}
