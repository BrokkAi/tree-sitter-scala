//! This crate provides Scala language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [LANGUAGE][] constant to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! use tree_sitter::Parser;
//!
//! let code = r#"
//! object Hello {
//!   def main(args: Array[String]) = {
//!     println("Hello, world")
//!   }
//! }
//! "#;
//! let mut parser = Parser::new();
//! let language = brokk_tree_sitter_scala::LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading Scala parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_scala() -> *const ();
}

/// The tree-sitter [`LanguageFn`][LanguageFn] for this grammar.
///
/// [LanguageFn]: https://docs.rs/tree-sitter-language/*/tree_sitter_language/struct.LanguageFn.html
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_scala) };

/// The content of the [`node-types.json`][] file for this grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const NODE_TYPES: &str = include_str!("../../src/node-types.json");

/// The syntax highlighting query for this language.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The local-variable syntax highlighting query for this language.
pub const LOCALS_QUERY: &str = include_str!("../../queries/locals.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::LANGUAGE.into())
            .expect("Error loading Scala parser");
    }

    #[test]
    fn export_selector_preserves_the_call_and_following_member() {
        let source = "class Runner { def run() = Export.export(project, file); def next() = 1 }";
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&super::LANGUAGE.into()).unwrap();
        let tree = parser.parse(source, None).unwrap();
        assert!(
            !tree.root_node().has_error(),
            "{}",
            tree.root_node().to_sexp()
        );

        let mut stack = vec![tree.root_node()];
        let mut selectors = Vec::new();
        let mut functions = Vec::new();
        while let Some(node) = stack.pop() {
            if node.kind() == "call_expression" {
                let function = node.child_by_field_name("function").unwrap();
                assert_eq!(function.kind(), "field_expression");
                let selector = function.child_by_field_name("field").unwrap();
                assert_eq!(selector.kind(), "identifier");
                selectors.push(selector.utf8_text(source.as_bytes()).unwrap());
            }
            if node.kind() == "function_definition" {
                functions.push(
                    node.child_by_field_name("name")
                        .unwrap()
                        .utf8_text(source.as_bytes())
                        .unwrap(),
                );
            }
            let mut cursor = node.walk();
            stack.extend(node.named_children(&mut cursor));
        }
        selectors.sort_unstable();
        functions.sort_unstable();
        assert_eq!(selectors, ["export"]);
        assert_eq!(functions, ["next", "run"]);
    }

    #[test]
    fn scala_three_export_keeps_its_declaration_node() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&super::LANGUAGE.into()).unwrap();
        let tree = parser.parse("export service.run", None).unwrap();
        assert!(
            !tree.root_node().has_error(),
            "{}",
            tree.root_node().to_sexp()
        );
        assert_eq!(
            tree.root_node().named_child(0).unwrap().kind(),
            "export_declaration"
        );
    }

    #[test]
    fn bundled_queries_compile() {
        let language = super::LANGUAGE.into();
        for query in [
            super::HIGHLIGHTS_QUERY,
            super::LOCALS_QUERY,
            include_str!("../../queries/indents.scm"),
            include_str!("../../queries/tags.scm"),
        ] {
            tree_sitter::Query::new(&language, query).unwrap();
        }
    }
}
