pub enum Transition {
    ActionTransition { target: ATNState, rule_index: isize, action_index: isize, ctx_dependent: bool },
    AtomTransition { target: ATNSate, label: isize },
    EpsilonTransition { target: ATNState, outermost_precedence_return: isize },
    SetTransition { target: ATNState, set: IntervalSet },
    NotSetTransition { target: ATNState, set: IntervalSet },
    PredicateTransition { target: ATNState, rule_index: isize, pred_index: isize, is_ctx_dependent: bool },
    PrecedencePredicateTransition { target: ATNState, precedence: isize },
    RangeTransition { target: ATNState, from: isize, to: isize },
    RuleTransition { target: ATNState, rule_index: isize, precedence: isize, follow_state: ATNState },
    WildcardTransition { target: ATNState }
}

impl Transition {

    fn target(&self) -> &ATNState {
        match self {
            Transition::ActionTransition {ref target, ..} => target,
            Transition::AtomTransition {ref target, ..} => target,
            Transition::EpsilonTransition {ref target, ..} => target,
            Transition::SetTransition {ref target, ..} => target,
            Transition::NotSetTransition {ref target, ..} => target,
            Transition::PredicateTransition {ref target, ..} => target,
            Transition::PrecedencePredicateTransition {ref target, ..} => target,
            Transition::RangeTransition {ref target, ..} => target,
            Transition::RuleTransition {ref target, ..} => target,
            Transition::WildcardTransition {ref target, ..} => target
        }
    }

    fn is_epsilon(&self) -> bool {
        match self {
            Transition::ActionTransition {..} => true,
            Transition::EpsilonTransition {..} => true,
            Transition::PredicateTransition {..} => true,
            Transition::PrecedencePredicateTransition {..} => true,
            Transition::RuleTransition {..} => true,
            _ => false
        }
    }

    fn label(&self) -> Option<IntervalSet> {
        match *self {
            Transition::AtomTransition {ref target, label} => Some(IntervalSet::one(label)),
            Transition::SetTransition {ref target, ref set} => Some(set.clone()),
            Transition::NotSetTransition {ref target, ref set} => Some(set.clone()),
            Transition::RangeTransition {ref target, from, to} => Some(IntervalSet::range(from, to)),
            _ => None
        }
    }

    fn matches(&self, symbol: isize, min_vocab_symbol: isize, max_vocab_symbol: isize) -> bool {
        match *self {
            Transition::AtomTransition {ref target, label} => label == symbol,
            Transition::SetTransition {ref target, ref set} => set.contains(symbol),
            Transition::NotSetTransition {ref target, ref set} => symbol >= min_vocab_symbol && symbol <= max_vocab_symbol && !set.contains(symbol),
            Transition::RangeTransition {ref target, from, to} => symbol >= from && symbol <= to,
            Transition::WildcardTransition {..} => symbol >= min_vocab_symbol && symbol <= max_vocab_symbol,
            _ => false
        }
    }

}
