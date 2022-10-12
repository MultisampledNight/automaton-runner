use {
    crate::{Conclusion, ExecutedAutomaton, Id},
    std::fmt,
};

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Conclusion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Conclusion::Accepted => write!(f, "accepted"),
            Conclusion::NotAtEndnode => {
                write!(f, "rejected, the final node was not marked as an end node")
            }
            Conclusion::AtCatchnode => {
                write!(f, "rejected, the final node was a catch node")
            }
            Conclusion::UnknownChar { at, input } => write!(
                f,
                "rejected, state `{}` didn't define what to do at character `{}`",
                at, input,
            ),
            Conclusion::UnknownTarget { from, through, target } => write!(
                f,
                "rejected, state `{}` defined a transition through `{}` to `{}`, which isn't defined anywhere",
                from, through, target,
            ),
        }
    }
}

impl fmt::Display for ExecutedAutomaton {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nodes visited: ")?;
        for (i, node_id) in self.path_taken.iter().enumerate() {
            if i != 0 {
                write!(f, " -> ")?;
            }
            write!(f, "{}", node_id)?;
        }

        write!(f, "\nconclusion: {}", self.conclusion)
    }
}
