use antlr::misc::interval::Interval;

pub trait ParseTree {

    fn parent(&self) -> &Option<ParseTree>;

    fn child(&self, i: isize) -> &Option<ParseTree>;

    fn child_count(&self) -> isize;

    fn accept<T>(&self, visitor: &ParseTreeVisitor<T>) -> T;

    fn text(&self) -> String;

    fn source_interval(&self) -> Interval;

}

pub trait ParseTreeVisitor<T> {

    fn visit(&self, tree: &ParseTree) -> T {
        return tree.accept(&self);
    }

    fn visit_children(&self, tree: &ParseTree) -> T {
        let mut result = default_result();

        let n = tree.child_count();
        let mut i = 0;
        while i < n {
            if !self.should_visit_next_child(tree, result) {
                break;
            }

            let child = tree.child(i);
            let child_result = child.accept(&self);
            result = self.aggregate_result(result, child_result);
        }

        return result;
    }

    fn visit_terminal(&self, node: &TerminalNode) -> T {
        return self.default_result();
    }

    fn visit_error(&self, node: &ErrorNode) -> T {
        return self.default_result();
    }

    fn should_visit_next_child(&self, node: &ParseTree, result: &T) -> bool {
        return true;
    }

    fn default_result(&self) -> T {
        return None;
    }

    fn aggregate_result(aggregate: &T, next: &T) -> T {
        return next;
    }

}

pub trait ParseTreeListener<T> {

    fn visit_terminal_node(&self) -> T;

    fn visit_error_node(&self) -> T;

    fn enter_every_rule(&self) -> T;

    fn exit_every_rule(&self) -> T;

}

pub trait TerminalNode: ParseTree {

    fn symbol(&self) -> Token;

}

pub trait ErrorNode: TerminalNode {}

pub struct TerminalNodeImpl {
    symbol: Token,
    parent: Option<ParseTree>
}

impl TerminalNode for TerminalNodeImpl {

    fn parent(&self) -> &Option<ParseTree> {
        return self.parent;
    }

    fn child(&self, i: isize) -> &Option<ParseTree> {
        return None;
    }

    fn child_count(&self) -> isize {
        return 0;
    }

    fn accept<T>(&self, visitor: &ParseTreeVisitor<T>) -> T {
        return visitor.visit_terminal(&self);
    }

    fn text(&self) -> String {
        return self.symbol.text();
    }

    fn source_interval(&self) -> Interval {
        return Interval::one(self.symbol.token_index());
    }

    fn symbol(&self) -> Token {
        return self.symbol;
    }
}

pub struct ErrorNodeImpl {
    symbol: Token,
    parent: Option<ParseTree>
}

impl TerminalNode for ErrorNodeImpl {

    fn parent(&self) -> &Option<ParseTree> {
        return self.parent;
    }

    fn child(&self, i: isize) -> &Option<ParseTree> {
        return None;
    }

    fn child_count(&self) -> isize {
        return 0;
    }

    fn accept<T>(&self, visitor: &ParseTreeVisitor<T>) -> T {
        return visitor.visit_error(&self);
    }

    fn text(&self) -> String {
        return self.symbol.text();
    }

    fn source_interval(&self) -> Interval {
        return Interval::one(self.symbol.token_index());
    }

    fn symbol(&self) -> Token {
        return self.symbol;
    }
}