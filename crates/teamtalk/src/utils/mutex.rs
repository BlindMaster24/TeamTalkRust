#[doc(hidden)]
#[repr(transparent)]
pub struct UnpoisonedMutex<T>(std::sync::Mutex<T>);

impl<T> UnpoisonedMutex<T> {
    #[doc(hidden)]
    pub fn new(value: T) -> Self {
        Self(std::sync::Mutex::new(value))
    }

    #[doc(hidden)]
    pub fn lock(&self) -> std::sync::MutexGuard<'_, T> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    #[doc(hidden)]
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().unwrap_or_else(|e| e.into_inner())
    }

    #[doc(hidden)]
    pub fn into_inner(self) -> T {
        self.0.into_inner().unwrap_or_else(|e| e.into_inner())
    }
}

impl<T: Default> Default for UnpoisonedMutex<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for UnpoisonedMutex<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}
