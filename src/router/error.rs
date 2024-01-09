pub trait DisplayableError {
    fn title(&self) -> String;
    fn body(&self) -> String;
}


