pub enum PollState<T> {
    Ready(T),
    NotReady,
}

pub trait Future {
    type Output;
    fn poll(&mut self) -> PollState<Self::Output>;
}
