#[derive(Clone, Debug, Default)]
pub enum VoterState {
    #[default]
    Ready,
    Vote,
    VotedPos,
    Reset,
}

impl VoterState {
    pub fn as_str(&self) -> &'static str {
        match self {
            VoterState::Ready => "Ready",
            VoterState::Vote => "Vote",
            VoterState::VotedPos => "VotedPos",
            VoterState::Reset => "Reset",
        }
    }
}
