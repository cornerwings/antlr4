use antlr::tree::parse_tree::ParseTree;

pub trait ParseTreeListenerr<T> {

    fn visit_terminal_node(&self) -> T;

    fn visit_error_node(&self) -> T;

    fn enter_every_rule(&self) -> T;

}