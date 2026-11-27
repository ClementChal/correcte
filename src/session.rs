#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct RequestId(u64);

pub(crate) struct Correction {
    pub(crate) id: RequestId,
    pub(crate) text: String,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Completion {
    Corrected,
    Failed,
}

#[derive(Default, PartialEq, Eq)]
enum State {
    #[default]
    Closed,
    Editing,
    Correcting(RequestId),
}

#[derive(Default)]
pub(crate) struct Session {
    state: State,
    text: String,
    last_request: u64,
}

impl Session {
    pub(crate) fn open(&mut self, text: String) {
        self.text = text;
        self.state = State::Editing;
    }

    pub(crate) fn close(&mut self) {
        self.text.clear();
        self.state = State::Closed;
    }

    pub(crate) fn text(&self) -> &str {
        &self.text
    }

    pub(crate) fn edit(&mut self, text: String) {
        if self.state == State::Editing {
            self.text = text;
        }
    }

    pub(crate) fn start_correction(&mut self) -> Option<Correction> {
        if self.state != State::Editing || self.text.trim().is_empty() {
            return None;
        }

        self.last_request += 1;
        let id = RequestId(self.last_request);
        self.state = State::Correcting(id);
        Some(Correction {
            id,
            text: self.text.clone(),
        })
    }

    pub(crate) fn complete_correction(
        &mut self,
        id: RequestId,
        result: Result<String, ()>,
    ) -> Option<Completion> {
        if self.state != State::Correcting(id) {
            return None;
        }

        self.state = State::Editing;
        Some(match result {
            Ok(corrected) => {
                self.text = corrected;
                Completion::Corrected
            }
            Err(()) => Completion::Failed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Session;

    #[test]
    fn correction_uses_current_text_and_prevents_edits_until_result() {
        let mut session = Session::default();
        session.open("copied".into());
        session.edit("edited\nà".into());

        let correction = session
            .start_correction()
            .expect("edited text is available");
        assert_eq!(correction.text, "edited\nà");
        session.edit("too late".into());
        assert_eq!(session.text(), "edited\nà");
        assert!(session.start_correction().is_none());

        session.complete_correction(correction.id, Ok("corrected\nà".into()));
        assert_eq!(session.text(), "corrected\nà");
        session.edit("edited again".into());
        assert_eq!(session.text(), "edited again");
    }

    #[test]
    fn failed_correction_preserves_text_and_allows_retry() {
        let mut session = Session::default();
        session.open("source".into());
        let failed = session
            .start_correction()
            .expect("source text is available");

        session.complete_correction(failed.id, Err(()));

        assert_eq!(session.text(), "source");
        assert!(session.start_correction().is_some());
    }

    #[test]
    fn reopening_discards_edits_and_ignores_old_results_during_new_correction() {
        for result in [Ok("obsolete correction".into()), Err(())] {
            let mut session = Session::default();
            session.open("clipboard before closing".into());
            session.edit("abandoned edit".into());
            let first = session
                .start_correction()
                .expect("first correction can start");
            session.close();
            assert!(session.text().is_empty());
            session.open("current clipboard".into());
            let second = session.start_correction().expect("new session can start");
            assert_ne!(first.id, second.id);

            assert_eq!(session.complete_correction(first.id, result), None);
            assert_eq!(session.text(), "current clipboard");
            assert!(session.start_correction().is_none());

            session.complete_correction(second.id, Ok("current correction".into()));
            assert_eq!(session.text(), "current correction");
        }
    }

    #[test]
    fn empty_or_closed_session_cannot_start_a_correction() {
        let mut session = Session::default();
        assert!(session.start_correction().is_none());

        session.open(" \n".into());
        assert!(session.start_correction().is_none());

        session.edit("text".into());
        session.close();
        assert!(session.start_correction().is_none());
    }
}
