use zbus::Connection;
mod zbus_greeter;
use zbus_greeter::MyGreeter1Proxy;

#[async_std::main]
async fn main() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let my_greeter = MyGreeter1Proxy::new(&connection).await?;
    dbg!(my_greeter.say_hello("world").await?);

    dbg!(my_greeter.set_greeter_name("asdf").await);
    dbg!(my_greeter.greeter_name().await);

    dbg!(my_greeter.failing_property().await);
    dbg!(my_greeter.could_fail().await);

    Ok(())
}
