#[derive(Default)]
pub(crate) struct Clipboard {
    handle: Option<arboard::Clipboard>,
}

impl Clipboard {
    pub(crate) fn read_text(&mut self) -> Result<Option<String>, arboard::Error> {
        match self.handle()?.get_text() {
            Ok(text) => Ok(Some(text)),
            Err(arboard::Error::ContentNotAvailable) => Ok(None),
            Err(error) => Err(error),
        }
    }

    pub(crate) fn write_text(&mut self, text: &str) -> Result<(), arboard::Error> {
        self.handle()?.set_text(text)
    }

    fn handle(&mut self) -> Result<&mut arboard::Clipboard, arboard::Error> {
        if self.handle.is_none() {
            self.handle = Some(arboard::Clipboard::new()?);
        }

        Ok(self
            .handle
            .as_mut()
            .expect("clipboard handle is initialized"))
    }
}
