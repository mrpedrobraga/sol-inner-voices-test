use tree_sitter::Parser;

fn main() {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_aster::language())
        .expect("Could not load Aster grammar.");

    let source = "[main]\n- Hello, World!";

    let result = parser.parse(source, None).unwrap();

    dbg!(result);
}
