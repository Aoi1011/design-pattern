use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;
    use std::sync::Arc;

    fn bad() {
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);

        std::thread::spawn(|| {
            x1.set(43);
        });

        let x2 = Arc::clone(&x);
        std::thread::spawn(|| {
            x2.set(44);
        });
    }
}