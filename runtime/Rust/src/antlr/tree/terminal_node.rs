use antlr::tree::parse_tree::ParseTree;
use antlr::token::Token;

pub trait TerminalNode<T> : ParseTree<T> {

    fn symbol(&self) -> &Token;

}