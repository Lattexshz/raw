pub trait AsRaw<T> {
    fn as_raw(&self) -> &T;
}

pub trait ToRaw<T> {
    fn to_raw(&self) -> T;
}

pub trait IntoRaw<T> {
    fn into_raw(self) -> T;
}

pub trait FromRaw<T> {
    fn from_raw(obj:T) -> Self;
}