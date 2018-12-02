#[derive(Fail, Debug)]
pub enum DeserializeError {
    #[fail(display = "{}({}) get nothing", attr, value)]
    SourceNotFound {
        attr: String,
        value: String
    },
    #[fail(display = "source({}) is empty", source)]
    SourceEmpty {
        source: String,
    }
}