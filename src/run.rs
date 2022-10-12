use crate::{
    Automaton,
    Conclusion::{self, *},
    ExecutedAutomaton, Id, Map, Node, Role,
};

impl Automaton {
    pub fn run(self, input: impl IntoIterator<Item = char>) -> ExecutedAutomaton {
        let result = input.into_iter().try_fold(
            InProcessAutomaton {
                current: self.start_at.clone(),
                path_taken: vec![self.start_at],
                last_char: None,
            },
            |state, ch| {
                let next_id = self
                    .nodes
                    .get(&state.current)
                    .ok_or_else(|| state.clone().fail_as_unknown_target())?
                    .transitions
                    .get(&ch)
                    .ok_or_else(|| state.clone().fail_as_unknown_char(ch))?
                    .clone();

                Ok(InProcessAutomaton {
                    current: next_id.clone(),
                    path_taken: state.path_taken.with_push(next_id),
                    last_char: Some(ch),
                })
            },
        );
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
        UnknownTarget {
            from: self.path_taken.last().unwrap().clone(),
            through: self.last_char.unwrap(),
            target: self.current.clone(),
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
