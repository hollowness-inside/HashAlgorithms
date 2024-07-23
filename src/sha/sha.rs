use std::marker::PhantomData;

pub struct Sha<T> {
    _t: PhantomData<T>,
}
