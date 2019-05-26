use antlr::tree::parse_tree::ParseTree;

pub trait ParseTreeVisitor<T> {

    fn visit(&self, tree: &ParseTree<T>) -> T;

    fn visit_children(&self, tree: &ParseTree<T>) -> T;

    fn visit_terminal(&self) -> T;

}