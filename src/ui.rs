use std::{cell::RefCell, rc::Rc, time::Duration};

use slint::{CloseRequestResponse, ComponentHandle, Timer};

use crate::{
    clipboard::Clipboard,
    session::{Completion, Session},
};

slint::include_modules!();

const SIMULATED_DELAY: Duration = Duration::from_millis(700);

pub(crate) fn run() -> Result<(), slint::PlatformError> {
    let editor = Editor::new()?;
    let mut clipboard = Clipboard::default();
    let mut session = Session::default();

    let text = match clipboard.read_text() {
        Ok(Some(text)) if !text.is_empty() => text,
        Ok(_) => {
            editor.set_status("Le presse-papiers ne contient pas de texte.".into());
            String::new()
        }
        Err(_) => {
            editor.set_status("Impossible de lire le presse-papiers.".into());
            String::new()
        }
    };
    session.open(text);
    editor.set_editor_text(session.text().into());

    let session = Rc::new(RefCell::new(session));
    connect_editing(&editor, &session);
    connect_correction(&editor, &session);
    connect_copy(&editor, &session, clipboard);
    connect_close(&editor, &session);
    editor.invoke_focus_editor();
    editor.run()
}

fn connect_editing(editor: &Editor, session: &Rc<RefCell<Session>>) {
    let session = Rc::clone(session);
    let editor_ref = editor.as_weak();
    editor.on_edit_text(move |text| {
        session.borrow_mut().edit(text.into());
        if let Some(editor) = editor_ref.upgrade() {
            editor.set_status("".into());
        }
    });
}

fn connect_correction(editor: &Editor, session: &Rc<RefCell<Session>>) {
    let session = Rc::clone(session);
    let editor_ref = editor.as_weak();
    editor.on_correct_text(move || {
        let Some(editor) = editor_ref.upgrade() else {
            return;
        };
        let Some(correction) = session.borrow_mut().start_correction() else {
            return;
        };

        editor.set_correcting(true);
        editor.set_status("Correction simulée en cours…".into());

        let session = Rc::clone(&session);
        let editor_ref = editor.as_weak();
        Timer::single_shot(SIMULATED_DELAY, move || {
            let Some(editor) = editor_ref.upgrade() else {
                return;
            };
            let result = format!("{}\n\n[Correction simulée]", correction.text);
            let mut session = session.borrow_mut();
            let Some(completion) = session.complete_correction(correction.id, Ok(result)) else {
                return;
            };

            match completion {
                Completion::Corrected => {
                    editor.set_editor_text(session.text().into());
                    editor.set_status("Correction simulée terminée.".into());
                }
                Completion::Failed => {
                    editor.set_status("La correction a échoué.".into());
                }
            }
            editor.set_correcting(false);
            editor.invoke_focus_editor();
        });
    });
}

fn connect_copy(editor: &Editor, session: &Rc<RefCell<Session>>, clipboard: Clipboard) {
    let clipboard = RefCell::new(clipboard);
    let session = Rc::clone(session);
    let editor_ref = editor.as_weak();
    editor.on_copy_text(move || {
        if let Some(editor) = editor_ref.upgrade() {
            let status = match clipboard.borrow_mut().write_text(session.borrow().text()) {
                Ok(()) => "Copié",
                Err(_) => "Impossible de copier le texte.",
            };
            editor.set_status(status.into());
        }
    });
}

fn connect_close(editor: &Editor, session: &Rc<RefCell<Session>>) {
    let window_session = Rc::clone(session);
    editor.window().on_close_requested(move || {
        window_session.borrow_mut().close();
        CloseRequestResponse::HideWindow
    });

    let session = Rc::clone(session);
    let editor_ref = editor.as_weak();
    editor.on_close_editor(move || {
        if let Some(editor) = editor_ref.upgrade() {
            match editor.hide() {
                Ok(()) => session.borrow_mut().close(),
                Err(_) => editor.set_status("Impossible de fermer la fenêtre.".into()),
            }
        }
    });
}
