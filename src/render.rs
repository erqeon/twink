use std::{io::{self, Write}};
use crossterm::{cursor::Hide, event::{EnableMouseCapture, DisableMouseCapture, Event, KeyCode, read}, execute, terminal::{disable_raw_mode, enable_raw_mode}, cursor::{Show}};
use crate::dom::{Arena, NodeType::{self}};

pub fn render(arena: &Arena) -> io::Result<()> {
	enable_raw_mode()?;

	let mut stdout = io::stdout();
	execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;

	execute!(stdout, EnableMouseCapture)?;
	execute!(stdout, Hide)?;

	stdout.flush()?;

	let root_id = 0;
	let mut current_id = Some(root_id);

	while let Some(id) = current_id {
		let node = &arena.nodes[id];

		match node.node_type {
			NodeType::Text(ref text) => {
				print!("{}", text);
			}

			NodeType::Element(ref data) => {
				if data.tag_name == "br" || data.tag_name == "p" || data.tag_name == "h1" {
					print!("\r\n");
				}
			}
		}

		if node.first_child.is_some() {
			current_id = node.first_child;
		}
		
		else if node.next_sibling.is_some() {
			current_id = node.next_sibling;
		}
		
		else {
			let mut parent_id = node.perent;
			let mut found_next = None;

			while let Some(p_id) = parent_id {
				let parent_node = &arena.nodes[p_id];
				if parent_node.next_sibling.is_some() {
					found_next = parent_node.next_sibling;
					break;
				}
				parent_id = parent_node.perent;
			}

			current_id = found_next;
		}
	}

	io::stdout().flush()?;

	loop {
		match read()? {
			Event::Key(key_event) => {
				stdout.flush()?;

				if key_event.code == KeyCode::Char('q') {
					break;
				}
			}
			_ => {}
		}
	}

	execute!(stdout, DisableMouseCapture)?;
    execute!(stdout, Show)?;
	disable_raw_mode()?;
	Ok(())
}
