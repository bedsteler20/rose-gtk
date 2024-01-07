pub trait DisplayableError {
    fn tile(&self) -> String;
    fn body(&self) -> String;
}


