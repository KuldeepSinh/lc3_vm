pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

pub enum ErrReason {
    FileNotFound,
}