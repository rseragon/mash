/// A pair struct to deonte the error and the give an exmplation for impl 

#[derive(Debug)]
pub struct ErrAndExpl<E> {
    pub err: E,
    pub expl: String
}

impl <E> ErrAndExpl<E> {
    pub fn new(err: E, expl: String) -> ErrAndExpl<E> {
        ErrAndExpl {
            err,
            expl
        }
    }
}
