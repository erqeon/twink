pub mod dom;
pub mod html;

fn main() {
    let source: String = std::fs::read_to_string("index.html").unwrap();
    let root_node = html::parse(source);

    println!("{:#?}", root_node);
}
