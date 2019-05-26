pub trait Transition {

    fn target(&self) -> ATNState;

    fn serialization_type(&self) -> isize;

    fn is_epsilon(&self) -> bool {
        return false;
    }

    fn label(&self) -> Option<IntervalSet> {
        return None;
    }

    fn matches(&self, symbol: isize, min_vocab_symbol: isize, max_vocab_symbol: isize) -> bool;
}

const EPSILON: isize = 1;
const RANGE: isize = 2;
const RULE: isize = 3;
const PREDICATE: isize = 4;
const ATOM: isize = 5;
const ACTION: isize	= 6;
const SET: isize = 7;
const NOT_SET: isize = 8;
const WILDCARD: isize = 9;
const PRECEDENCE: isize = 10;

pub struct ActionTransition {
    target: ATNState,
    rule_index: isize,
    action_index: isize,
    is_ctx_dependent: bool
}

impl Transition for ActionTransition {

    fn target(&self) -> ATNState {
        return self.target;
    }

    fn serialization_type(&self) -> isize {
        return ACTION;
    }

    fn is_epsilon(&self) -> bool {
        return true;
    }

    fn matches(&self, symbol: isize, min_vocab_symbol: isize, max_vocab_symbol: isize) -> bool {
        return false;
    }

}

pub struct AtomTransition {
    target: ATNState,
    label: isize
}

impl Transition for AtomTransition {

    fn target(&self) -> ATNState {
        return self.target;
    }

    fn serialization_type(&self) -> isize {
        return ATOM;
    }

    fn label(&self) -> Option<IntervalSet> {
        return Some(IntervalSet::one(label));
    }

    fn matches(&self, symbol: isize, min_vocab_symbol: isize, max_vocab_symbol: isize) -> bool {
        return label == symbol;
    }

}