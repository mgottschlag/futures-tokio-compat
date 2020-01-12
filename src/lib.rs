use pin_project::pin_project;

//mod error;
mod io;
//mod spawn;

#[pin_project]
#[derive(Copy, Clone, Debug)]
pub struct Compat<T> {
    #[pin]
    inner: T,
}

impl<T> Compat<T> {
    pub fn new(inner: T) -> Self {
        Compat { inner }
    }
}

pub(crate) trait LocalFrom<T> {
    fn from(t: T) -> Self;
}
