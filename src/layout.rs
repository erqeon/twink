use crate::dom::{self, LayoutArena};
use std::default::Default;

pub use self::BoxType::{AnonymousBlock, InlineNode, BlockNode};

#[derive(Clone, Copy, Default, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Dimensions {
    pub content: Rect,
    pub padding: EdgeSizes,
    pub border: EdgeSizes,
    pub margin: EdgeSizes,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

#[derive(Debug)]
pub struct LayoutBox {
    pub dimensions: Dimensions,
    pub box_type: BoxType,
    pub parent: Option<usize>,
    pub first_child: Option<usize>,
    pub next_sibling: Option<usize>,
    pub dom_node_id: usize,
}

#[derive(Debug)]
pub enum BoxType {
    BlockNode,
    InlineNode,
    AnonymousBlock,
}

impl LayoutBox {
    fn new(box_type: BoxType, node_id: usize) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(),
            dom_node_id: node_id,
            parent: None,
            first_child: None,
            next_sibling: None,
        }
    }
}

pub fn layout_tree(
    layout_arena: &mut LayoutArena,
    dom_arena: &dom::Arena,
    dom_root_id: usize,
    mut _containing_block: Dimensions
) -> usize {
    _containing_block.content.height = 0.0;

    let root_id = build_layout_tree(layout_arena, dom_arena, dom_root_id);
    
    //layout_arena.boxes[root_id].layout(containing_block, layout_arena);
    
    return root_id;
}

fn build_layout_tree(
    layout_arena: &mut LayoutArena, 
    dom_arena: &dom::Arena, 
    dom_node_id: usize
) -> usize {
    let dom_node = &dom_arena.nodes[dom_node_id];
    
    let box_type = match &dom_node.node_type {
        dom::NodeType::Element(element_data) => {
            if element_data.tag_name == "html" || element_data.tag_name == "h1" || element_data.tag_name == "p" {
                BoxType::BlockNode
            } else {
                BoxType::InlineNode
            }
        }
        dom::NodeType::Text(_) => BoxType::InlineNode,
    };

    let root_box = LayoutBox::new(box_type, dom_node_id);
    layout_arena.boxes.push(root_box);
    let current_box_id = layout_arena.boxes.len() - 1;

    let mut first_child_id: Option<usize> = None;
    let mut previous_child_id: Option<usize> = None;

    let mut child_id_opt = dom_node.first_child;
    while let Some(child_dom_id) = child_id_opt {
        let current_child_box_id = build_layout_tree(layout_arena, dom_arena, child_dom_id);
        
        layout_arena.boxes[current_child_box_id].parent = Some(current_box_id);

        if first_child_id.is_none() {
            first_child_id = Some(current_child_box_id);
            layout_arena.boxes[current_box_id].first_child = Some(current_child_box_id);
        } else if let Some(prev_id) = previous_child_id {
            layout_arena.boxes[prev_id].next_sibling = Some(current_child_box_id);
        }

        previous_child_id = Some(current_child_box_id);
        child_id_opt = dom_arena.nodes[child_dom_id].next_sibling;
    }
    return current_box_id;
}

impl Rect {
    pub fn expanded_by(self, edge: EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

impl Dimensions {
    pub fn padding_box(self) -> Rect {
        self.content.expanded_by(self.padding)
    }

    pub fn border_box(self) -> Rect {
        self.padding_box().expanded_by(self.border)
    }

    pub fn margin_box(self) -> Rect {
        self.border_box().expanded_by(self.margin)
    }
}

#[allow(dead_code)]
fn sum<I>(iter: I) -> f32 where I: Iterator<Item=f32> {
    iter.fold(0., |a, b| a + b)
}
