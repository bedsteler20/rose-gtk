use gtk::glib;

use gtk::glib::once_cell;
use tokio::runtime::Runtime;

pub static RUNTIME: once_cell::sync::Lazy<Runtime> =
    once_cell::sync::Lazy::new(|| Runtime::new().unwrap());

/// Spawn a future in the tokio runtime and call the callback in the glib main context.
///
/// # Arguments
/// * `future` - The future to spawn in the tokio runtime.
/// * `callback` - The callback to call in the glib main context when the future is done.
///
/// # Example
/// ```
/// let label = gtk::Label::new(Some("Hello"));
/// let foo = "foo".to_string();
/// rose::spawn_async(async move {
///    // This is a blocking call, since its running in the tokio runtime. It will not block the
///    // the application since its running in a separate thread. However it GObjects can't be
///    // accessed hear
///    reqwest::get(format!("https://www.rust-lang.org/{}", foo))
///         .await
///         .unwrap()
///         .text()
///         .await
///         .unwrap();
/// }, move |resp| {
///     // This is called in the glib main context, so GObjects can be safely accessed here.
///     // Glib will poll the tokio runtime for the future to be done. This is done whenever
///     // the main thread is idle so it will not block the application until the future is done.
///     label.set_text(&resp);
/// });
/// ```
pub fn spawn_async<T>(
    future: impl std::future::Future<Output = T> + 'static + Send,
    callback: impl FnOnce(T) + 'static + Send,
) where
    T: Send + 'static,
{
    let handel = RUNTIME.spawn(future);
 
    glib::spawn_future_local(async move {
        callback(handel.await.unwrap());
    });
}

/// Spawn a background thread that can send multiple values back to the main thread.
/// The callback will be called in the glib main context when a value is sent.
/// GObjects can be safely cloned and used by this callback since it will be called on the main
/// thread.
///
/// # Arguments
/// * `thread` - The thread to spawn. The thread will be given a `Sender` that can be used to send
/// values back to the main thread.
/// * `callback` - The callback to call in the glib main context when a value is sent.
///
/// # Example
/// ```
/// let label = gtk::Label::new(Some("Hello"));
/// let foo = "foo".to_string();
/// rose::spawn_background_thread(move |sender| {
///    // This is a blocking call, since its running in a separate thread. It will not block the
///   // the application since its running in a separate thread. However it GObjects can't be
///   // accessed hear
///   let mut i = 0;
///   loop {
///      i += 1;
///      thread::sleep(Duration::from_secs(1));
///      sender.send(i);
///  }
/// }, move |resp| {
///    // This is called in the glib main context, so GObjects can be safely accessed here.
///   label.set_text(&resp);
/// });
pub fn spawn_background_thread<T>(
    thread: impl FnOnce(async_channel::Sender<T>) -> T + Send + 'static,
    callback: impl Fn(T) + Send + 'static,
) where
    T: Send + 'static,
{
    let (sender, receiver) = async_channel::bounded::<T>(1);

    std::thread::spawn(move || {
        thread(sender.clone());
        sender.close();
    });

    glib::spawn_future_local(async move {
        while let Ok(value) = receiver.recv().await {
            callback(value);
        }
    });
}
