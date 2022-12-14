use crate::{
    Automaton,
    Conclusion::{self, *},
    ExecutedAutomaton, Id, Map, Node, Role,
};

impl Automaton {
    pub fn run(self, input: impl IntoIterator<Item = char>) -> ExecutedAutomaton {
        // try_fold makes the model of "run one step" way easier than a for-loop with many distinct
        // variables
        let initial_state = InProcessAutomaton {
            current: self.start_at.clone(),
            path_taken: vec![self.start_at],
            last_char: None,
        };

        let result = input.into_iter().try_fold(initial_state, |state, ch| {
            let current_node = self
                .nodes
                .get(&state.current)
                .ok_or_else(|| state.clone().fail_as_unknown_target())?;

            // do note the next ID might not be defined -- then it'll fail next step in ^
            let next_id = current_node
                .transitions
                .get(&ch)
                .ok_or_else(|| state.clone().fail_as_unknown_char(ch))?
                .clone();

            Ok(InProcessAutomaton {
                current: next_id.clone(),
                path_taken: state.path_taken.with_push(next_id),
                last_char: Some(ch),
            })
        });
        match result {
            Ok(unfinished) => unfinished.finalize(&self.nodes),
            Err(executed_with_err) => executed_with_err,
        }
    }
}

trait WithPush<T> {
    fn with_push(self, item: T) -> Self;
}

impl<T> WithPush<T> for Vec<T> {
    fn with_push(mut self, item: T) -> Self {
        self.push(item);
        self
    }
}

#[derive(Clone, Debug)]
struct InProcessAutomaton {
    current: Id,
    path_taken: Vec<Id>,
    last_char: Option<char>,
}

impl InProcessAutomaton {
    fn unknown_target_conclusion(&self) -> Conclusion {
        let [.., original, current] = &self.path_taken[..] else { unreachable!() };
        UnknownTarget {
            from: original.clone(),
            through: self.last_char.unwrap(),
            target: current.clone(),
        }
    }
}

impl InProcessAutomaton {
    fn fail_as_unknown_target(self) -> ExecutedAutomaton {
        ExecutedAutomaton {
            conclusion: self.unknown_target_conclusion(),
            path_taken: self.path_taken,
        }
    }

    fn fail_as_unknown_char(self, ch: char) -> ExecutedAutomaton {
        ExecutedAutomaton {
            conclusion: UnknownChar {
                at: self.current,
                input: ch,
            },
            path_taken: self.path_taken,
        }
    }

    fn finalize(self, nodes: &Map<Id, Node>) -> ExecutedAutomaton {
        ExecutedAutomaton {
            conclusion: match nodes.get(&self.current).map(|final_node| final_node.role) {
                Some(Role::End) => Accepted,
                Some(Role::None) => NotAtEndnode,
                Some(Role::Catch) => AtCatchnode,
                None => self.unknown_target_conclusion(),
            },
            path_taken: self.path_taken,
        }
    }
}
