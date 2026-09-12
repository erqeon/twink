/* surprisingly, it works */

pub mod dom;
pub mod html;
pub mod css;
pub mod layout;
pub mod render;

#[allow(dead_code)]
fn print_tree(arena: &dom::Arena, node_id: usize, depth: usize) {
    let node = &arena.nodes[node_id];
    let indent = "  ".repeat(depth);

    match &node.node_type {
        dom::NodeType::Text(text_content) => {
            let trimmed = text_content.trim();
            if !trimmed.is_empty() {
                println!("{}\"{}\"", indent, trimmed);
            }
        }
        dom::NodeType::Element(element_data) => {
            if !element_data.attrs.is_empty() {
                let attrs_str: Vec<String> = element_data.attrs.iter().map(|(k, v)| format!("{}=\"{}\"", k, v)).collect();

                println!("{}{} [{}]", indent, element_data.tag_name, attrs_str.join(", "));
            } else {
                println!("{}{}", indent, element_data.tag_name);
            }
        }
    }

    if let Some(child_id) = node.first_child {
        print_tree(arena, child_id, depth + 1);
    }

    if let Some(sibling_id) = node.next_sibling {
        print_tree(arena, sibling_id, depth);
    }
}

fn main() {
    let source: String = std::fs::read_to_string("examples/index.html")
        .expect("not found index.html, create file!");
    let (arena, _root_id) = html::parse(source);
    
    // print_tree(&arena, root_id, 0);
    let _ = render::render(&arena);
}
