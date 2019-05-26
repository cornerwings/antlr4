use antlr::tree::parse_tree::ParseTree;
use antlr::rule_context::RuleContext;

pub trait RuleNode<T> : ParseTree<T> {

    fn get_rule_context(&self) -> &RuleContext;

}