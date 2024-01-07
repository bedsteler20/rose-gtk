use gtk::glib;

pub fn channel<T>() -> (Sender<T>, Receiver<T>)
where
    T: Send + 'static,
{
    let (sender, receiver) = async_channel::unbounded::<T>();
    let sender = Sender { inner: sender };
    let receiver = Receiver { inner: receiver };
    (sender, receiver)
}

pub struct Sender<T> {
    inner: async_channel::Sender<T>,
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) {
        self.inner.try_send(value).unwrap();
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.inner.close();
    }
}

pub struct Receiver<T>
where
    T: Send + 'static,
{
    inner: async_channel::Receiver<T>,
}

impl<T> Receiver<T>
where
    T: Send + 'static,
{
    pub fn attach<F>(&self, callback: F)
    where
        F: Fn(T) + Send + 'static,
    {
        let inner = self.inner.clone();
        glib::spawn_future_local(async move {
            while let Ok(value) = inner.recv().await {
                callback(value);
            }
        });
    }
}

impl<T> Clone for Receiver<T>
where
    T: Send + 'static,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Drop for Receiver<T>
where
    T: Send + 'static,
{
    fn drop(&mut self) {
        self.inner.close();
    }
}
