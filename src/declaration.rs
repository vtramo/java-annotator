use crate::annotator::JavaAnnotation;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct JavaTypeDeclaration {
    name: String,
    marker_annotations: Vec<String>,
    start_position: tree_sitter::Point,
    start_byte: usize,
    tot_whitespaces: usize,
}

impl JavaTypeDeclaration {
    pub fn contains_marker_annotation(&self, annotation_name: &JavaAnnotation) -> bool {
        let annotation_name = annotation_name.trim_start_matches('@');
        self.marker_annotations
            .contains(&annotation_name.to_string())
    }

    pub fn tot_whitespaces(&self) -> usize {
        self.tot_whitespaces
    }

    pub fn start_byte(&self) -> usize {
        self.start_byte
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn build_java_declaration(node: &tree_sitter::Node, src: &str) -> JavaTypeDeclaration {
    if node.kind() != "class_declaration" && node.kind() != "interface_declaration" {}

    let marker_annotations = find_all_marker_annotation_nodes(node);
    let marker_annotation_names = extract_all_names(&marker_annotations, src);

    let start_byte = node.start_byte();
    let tot_whitespaces = count_whitespaces_reverse(&src[..start_byte]);
    JavaTypeDeclaration {
        name: extract_name(node, src).unwrap(),
        marker_annotations: marker_annotation_names,
        start_position: node.start_position(),
        start_byte,
        tot_whitespaces,
    }
}

fn find_all_marker_annotation_nodes<'a>(
    node: &tree_sitter::Node<'a>,
) -> Vec<tree_sitter::Node<'a>> {
    let mut cursor = node.walk();

    let modifiers_node = match node
        .children(&mut cursor)
        .find(|child| child.kind() == "modifiers")
    {
        Some(n) => n,
        None => return Vec::new(),
    };

    let mut modifier_cursor = modifiers_node.walk();

    modifiers_node
        .children(&mut modifier_cursor)
        .filter(|child| child.kind() == "marker_annotation")
        .collect()
}

fn extract_all_names(nodes: &[tree_sitter::Node], src: &str) -> Vec<String> {
    nodes
        .iter()
        .map(|node| extract_name(node, src))
        .flatten()
        .collect()
}

fn extract_name(node: &tree_sitter::Node, src: &str) -> Option<String> {
    let identifier_node = node
        .children(&mut node.walk())
        .find(|child| child.kind() == "identifier")?;

    identifier_node
        .utf8_text(src.as_bytes())
        .ok()
        .map(|s| s.to_string())
}

fn count_whitespaces_reverse(s: &str) -> usize {
    s.chars()
        .rev()
        .take_while(|c| c.is_whitespace() && *c != '\n')
        .count()
}
