/* took from https://limpet.net/mbrubeck/2014/08/11/toy-layout-engine-2.html */

use crate::dom;
use std::collections::HashMap;

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos ..].starts_with(s)
    }

    fn expect(&mut self, s: &str) {
        if self.starts_with(s) {
            self.pos += s.len();
        } else {
            panic!("Expected {:?} at byte {} but it was not found", s, self.pos);
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        return c;
    }

    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char());
        }
        return result;
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..'9'))
    }

    fn parse_node(&mut self, arena: &mut dom::Arena) -> usize {
        if self.starts_with("<") {
            self.parse_element(arena)
        } else {
            self.parse_text(arena)
        }
    }

    fn parse_text(&mut self, arena: &mut dom::Arena) -> usize {
        let text_content = self.consume_while(|c| c != '<');
        let node = dom::text(text_content);
        arena.nodes.push(node);
        return arena.nodes.len() - 1;
    }

    fn parse_element(&mut self, arena: &mut dom::Arena) -> usize {
        self.expect("<");
        let tag_name = self.parse_name();
        let attrs = self.parse_attributes();
        self.expect(">");

        let node = dom::elem(tag_name.clone(), attrs);
        arena.nodes.push(node);
        let current_id = arena.nodes.len() - 1;

        let first_child_id = self.parse_nodes(arena, current_id);
        arena.nodes[current_id].first_child = first_child_id;

        self.expect("</");
        self.expect(&tag_name);
        self.expect(">");

        return current_id;
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_attr_value();
        return (name, value);
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert_eq!(open_quote, close_quote);
        return value;
    }

    fn parse_attributes(&mut self) -> dom::AttrMap {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == '>' {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        return attributes;
    }

    fn parse_nodes(&mut self, arena: &mut dom::Arena, parent_id: usize) -> Option<usize> {
        let mut first_child_id: Option<usize> = None;
        let mut previous_child_id: Option<usize> = None;

        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }

            if self.input[self.pos..].starts_with("<!--") {
                self.parse_comment();
                continue;
            }

            let current_child_id = self.parse_node(arena);
            arena.nodes[current_child_id].perent = Some(parent_id);

            if first_child_id.is_none() {
                first_child_id = Some(current_child_id);
            } else if let Some(prev_id) = previous_child_id {
                arena.nodes[prev_id].next_sibling = Some(current_child_id);
            }

            previous_child_id = Some(current_child_id);
        }
        return first_child_id;
    }

    fn parse_comment(&mut self) {
        if self.input[self.pos..].starts_with("<!--") {
            self.pos += 4;

            while !self.input[self.pos..].starts_with("-->") && self.pos < self.input.len() {
                self.consume_char();
            }

            if self.pos < self.input.len() {
                self.pos += 3;
            }
        }
    }
}

pub fn parse(source: String) -> (dom::Arena, usize) {
    let mut arena = dom::Arena { nodes: Vec::new() };
    let mut parser = Parser { pos: 0, input: source };

    let root_id = parser.parse_node(&mut arena);
    return (arena, root_id);
}