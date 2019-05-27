pub enum ATNState {
    BasicState {
        transitions: Vec<Transition>,
        epsilon_only_transitions: bool,
        atn: ATN,
        state_number: isize,
        rule_index: isize,
        next_token_with_rule: IntervalSet
    },
    DecisionState {
        transitions: Vec<Transition>,
        epsilon_only_transitions: bool,
        atn: ATN,
        state_number: isize,
        rule_index: isize,
        next_token_with_rule: IntervalSet,
        decision: isize,
        non_greedy: bool
    },
    BlockStartState {
        transitions: Vec<Transition>,
        epsilon_only_transitions: bool,
        atn: ATN,
        state_number: isize,
        rule_index: isize,
        next_token_with_rule: IntervalSet,
        decision: isize,
        non_greedy: bool,
        end_state: Box<ATNState>
    },
    BlockEndState {
        transitions: Vec<Transition>,
        epsilon_only_transitions: bool,
        atn: ATN,
        state_number: isize,
        rule_index: isize,
        next_token_with_rule: IntervalSet,
        decision: isize,
        non_greedy: bool,
        start_state: Box<ATNState>
    },
}

impl ATNState {

    fn add_transition(&mut self, t: Transition) {
        match self {
            ATNState::BasicState {ref mut transitions, ref mut epsilon_only_transition, ..} |
            ATNState::DecisionState {ref mut transitions, ref mut epsilon_only_transition, ..} => {
                if transitions.is_empty() {
                    *epsilon_only_transition = t.is_epsilon();
                } else if epsilon_only_transition != t.is_epsilon() {
                    *epsilon_only_transition = false;
                }

                let existing = transitions.iter().find(|e| {
                    if t.target().state_number() == e.target().state_number() {
                        if (t.label().is_some() && e.label().is_some() && t.label() == e.label()) || (t.is_epsilon() && e.is_epsilon()) {
                            true
                        }
                    }

                    false
                });

                if existing.is_none() {
                    transitions.push(t);
                }
            }
        }
    }

    fn num_transitions(&self) -> isize {
        match self {
            ATNState::BasicState {ref mut transitions, ..} |
            ATNState::DecisionState {ref mut transitions, ..} => transitions.len()
        }
    }

    fn get_transition(&mut self, i: isize) {
        match self {
            ATNState::BasicState {ref mut transitions, ..} |
            ATNState::DecisionState {ref mut transitions, ..} => transitions[i]
        }
    }

    fn set_transition(&mut self, i: isize, t: Transition) {
        match self {
            ATNState::BasicState {ref mut transitions, ..} |
            ATNState::DecisionState {ref mut transitions, ..} => transitions.insert(i, t)
        }
    }

    fn remove_transition(&mut self, i: isize) {
        match self {
            ATNState::BasicState {ref mut transitions, ..} |
            ATNState::DecisionState {ref mut transitions, ..} => transitions.remove(i)
        }
    }

}