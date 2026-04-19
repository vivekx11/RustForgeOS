//! Custom Future trait

pub trait Future {
    type Output;
    
    fn poll(&mut self) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
