use gtk::glib;

/// A channel for sending and receiving values.
pub struct Channel;
impl Channel {
    /// Create a new channel. The sender can be cloned and sent to other threads. The receiver
    /// can be attached to a callback. that callback will be called on the main thread when a
    /// value is sent.
    /// 
    /// # Example
    /// ```
    /// let label = gtk::Label::new(Some("Hello"));
    /// let (sender, receiver) = rose::Channel::new();
    /// 
    /// RUNTIME.spawn(async move {
    ///    sender.send("Hello from async land".to_string());
    /// });
    /// 
    /// receiver.attach(move |resp| {
    ///   label.set_text(&resp);
    /// });
    pub fn new<T>() -> (Sender<T>, Receiver<T>)
    where
        T: Send + 'static,
    {
        let (sender, receiver) = async_channel::unbounded::<T>();
        let sender = Sender { inner: sender };
        let receiver = Receiver { inner: receiver };
        (sender, receiver)
    }
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
    /// Attach a callback to the receiver. The callback will be called on the main thread when a
    /// value is sent. GObjects can be safely cloned and used by this callback since it will be
    /// called on the main thread.
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
