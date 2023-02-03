use zbus::{dbus_interface, fdo, ConnectionBuilder, Result, SignalContext};

use event_listener::Event;

struct Greeter {
    name: String,
    done: Event,
}

#[dbus_interface(name = "org.zbus.MyGreeter1")]
impl Greeter {
    async fn say_hello(&self, name: &str) -> String {
        format!("Hello {}!", name)
    }

    // Rude!
    async fn go_away(&self, #[zbus(signal_context)] ctxt: SignalContext<'_>) -> fdo::Result<()> {
        Self::greeted_everyone(&ctxt).await?;
        self.done.notify(1);

        Ok(())
    }

    /// a failing property
    #[dbus_interface(property)]
    async fn failing_property(&self) -> Result<&str> {
        Err(zbus::Error::Unsupported)
    }

    /// A "GreeterName" property.
    #[dbus_interface(property)]
    async fn greeter_name(&self) -> &str {
        &self.name
    }

    /// A setter for the "GreeterName" property.
    ///
    /// Additionally, a `greeter_name_changed` method has been generated for you if you need to
    /// notify listeners that "GreeterName" was updated. It will be automatically called when
    /// using this setter.  
    #[dbus_interface(property)]
    async fn set_greeter_name(&mut self, name: String) {
        self.name = name;
    }

    /// A signal; the implementation is provided by the macro.
    #[dbus_interface(signal)]
    async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()>;
}

// Although we use `async-std` here, you can use any async runtime of choice.
#[async_std::main]
async fn main() -> Result<()> {
    let greeter = Greeter {
        name: "GreeterName".to_string(),
        done: event_listener::Event::new(),
    };
    let done_listener = greeter.done.listen();
    let _connection = ConnectionBuilder::session()?
        .name("org.zbus.MyGreeter")?
        .serve_at("/org/zbus/MyGreeter", greeter)?
        .build()
        .await?;
    done_listener.wait();

    Ok(())
}
