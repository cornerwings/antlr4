use antlr::misc::interval::Interval;
use antlr::tree::parse_tree_visitor::ParseTreeVisitor;

pub trait ParseTree<T> {

    fn accept(&self, visitor: &ParseTreeVisitor<T>) -> T;

    fn text(&self) -> String;

    fn source_interval(&self) -> Interval;

}