use {
    crate::{Automaton, Id, Map, Node, Role},
    std::str::FromStr,
    thiserror::Error,
};

#[derive(Debug, Error)]
#[error("parse error at line {on_line}: {reason}")]
pub struct ParseError {
    pub on_line: usize,
    pub reason: ParseErrorReason,
}

#[derive(Debug, Error)]
pub enum ParseErrorReason {
    #[error("no chars found before `=>`")]
    MissingChars,
    #[error("no target given after `=>`")]
    MissingTarget,
    #[error("reference was given while not being under any section")]
    NotInSection,
    #[error("char between `|` was empty")]
    EmptyChar,
    #[error("no start state specified; specify one through suffixing it with ` # start`")]
    NoStartState,
    #[error("unknown modifier: `{0}`")]
    UnknownModifier(String),
}

#[derive(Copy, Clone)]
struct Context<'a> {
    on_line: usize,
    line: &'a str,
}

impl Context<'_> {
    fn error(&self, reason: ParseErrorReason) -> ParseError {
        ParseError {
            on_line: self.on_line,
            reason,
        }
    }
}

trait OptionExt<T>
where
    Self: Sized,
{
    fn ok_or_parse_error(self, ctx: Context, reason: ParseErrorReason) -> Result<T, ParseError>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok_or_parse_error(self, ctx: Context, reason: ParseErrorReason) -> Result<T, ParseError> {
        self.ok_or_else(|| ctx.error(reason))
    }
}

impl FromStr for Automaton {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: Map<Id, Node> = Map::new();

        let mut current_id = None;
        let mut start_id = None;

        for ctx in s
            .lines()
            .map(str::trim)
            .filter(|line| !(line.is_empty() || line.starts_with("//")))
            .enumerate()
            .map(|(on_line, line)| Context { on_line, line })
        {
            let line_parts: Vec<_> = ctx.line.split("=>").map(str::trim).collect();
            if let [pattern, target, ..] = line_parts[..] {
                // line is about a rule for when to switch states
                if pattern.is_empty() {
                    return Err(ctx.error(ParseErrorReason::MissingChars));
                }

                let chars = pattern
                    .chars()
                    .filter(|ch| !char::is_whitespace(*ch))
                    .collect::<Vec<_>>();

                if target.is_empty() {
                    return Err(ctx.error(ParseErrorReason::MissingTarget));
                }
                let target = Id(target.to_string());

                let new_next_entries = chars.into_iter().map(|ch| (ch, target.clone()));

                let current_id = current_id
                    .as_ref()
                    .ok_or_parse_error(ctx, ParseErrorReason::NotInSection)?;
                nodes
                    .get_mut(current_id)
                    .expect("section start branch noted ID without creating entry")
                    .transitions
                    .extend(new_next_entries)
            } else {
                // line starts a new section
                let mut line_parts = ctx.line.split('#').map(str::trim);

                let id = Id(line_parts.next().unwrap().to_string());
                current_id = Some(id.clone());

                let mut node = Node {
                    transitions: Map::new(),
                    role: Role::None,
                };

                for modifier in line_parts {
                    match modifier {
                        // start isn't a dedicated role, since the start node could also be an end
                        // node
                        "start" => start_id = Some(id.clone()),
                        "end" => node.role = Role::End,
                        "catch" => node.role = Role::Catch,
                        modifier => {
                            return Err(
                                ctx.error(ParseErrorReason::UnknownModifier(modifier.to_string()))
                            )
                        }
                    }
                }

                nodes.insert(id, node);
            }
        }

        Ok(Automaton {
            nodes,
            start_at: start_id.ok_or_else(|| ParseError {
                on_line: 0,
                reason: ParseErrorReason::NoStartState,
            })?,
        })
    }
}
