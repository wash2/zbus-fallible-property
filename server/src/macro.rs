#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use zbus::{dbus_interface, fdo, ConnectionBuilder, Result, SignalContext};
use event_listener::Event;
struct Greeter {
    name: String,
    done: Event,
}
impl Greeter {
    async fn say_hello(&self, name: &str) -> String {
        {
            let res = ::alloc::fmt::format(
                ::core::fmt::Arguments::new_v1(
                    &["Hello ", "!"],
                    &[::core::fmt::ArgumentV1::new_display(&name)],
                ),
            );
            res
        }
    }
    async fn go_away(&self, ctxt: SignalContext<'_>) -> fdo::Result<()> {
        Self::greeted_everyone(&ctxt).await?;
        self.done.notify(1);
        Ok(())
    }
    /// a failing property
    async fn failing_property(&self) -> Result<&str> {
        Err(zbus::Error::Unsupported)
    }
    /// A "GreeterName" property.
    async fn greeter_name(&self) -> &str {
        &self.name
    }
    /// A setter for the "GreeterName" property.
    ///
    /// Additionally, a `greeter_name_changed` method has been generated for you if you need to
    /// notify listeners that "GreeterName" was updated. It will be automatically called when
    /// using this setter.
    async fn set_greeter_name(&mut self, name: String) {
        self.name = name;
    }
    /// A signal; the implementation is provided by the macro.
    async fn greeted_everyone(ctxt: &SignalContext<'_>) -> Result<()> {
        ctxt.connection()
            .emit_signal(
                ::std::option::Option::None::<()>,
                ctxt.path(),
                <Greeter as ::zbus::Interface>::name(),
                "GreetedEveryone",
                &(),
            )
            .await
    }
}
impl Greeter {
    pub async fn failing_property_changed(
        &self,
        signal_context: &::zbus::SignalContext<'_>,
    ) -> ::zbus::Result<()> {
        let mut changed = ::std::collections::HashMap::new();
        let value = <::zbus::zvariant::Value as ::std::convert::From<
            _,
        >>::from(self.failing_property().await?);
        changed.insert("FailingProperty", &value);
        ::zbus::fdo::Properties::properties_changed(
                signal_context,
                ::zbus::names::InterfaceName::from_static_str_unchecked(
                    "org.zbus.MyGreeter1",
                ),
                &changed,
                &[],
            )
            .await
    }
    pub async fn greeter_name_changed(
        &self,
        signal_context: &::zbus::SignalContext<'_>,
    ) -> ::zbus::Result<()> {
        let mut changed = ::std::collections::HashMap::new();
        let value = <::zbus::zvariant::Value as ::std::convert::From<
            _,
        >>::from(self.greeter_name().await);
        changed.insert("GreeterName", &value);
        ::zbus::fdo::Properties::properties_changed(
                signal_context,
                ::zbus::names::InterfaceName::from_static_str_unchecked(
                    "org.zbus.MyGreeter1",
                ),
                &changed,
                &[],
            )
            .await
    }
}
impl ::zbus::Interface for Greeter {
    fn name() -> ::zbus::names::InterfaceName<'static> {
        ::zbus::names::InterfaceName::from_static_str_unchecked("org.zbus.MyGreeter1")
    }
    #[allow(
        clippy::async_yields_async,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn get<'life0, 'life1, 'async_trait>(
        &'life0 self,
        property_name: &'life1 str,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = ::std::option::Option<
                    ::zbus::fdo::Result<::zbus::zvariant::OwnedValue>,
                >,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret)
                = ::core::option::Option::None::<
                    ::std::option::Option<
                        ::zbus::fdo::Result<::zbus::zvariant::OwnedValue>,
                    >,
                > {
                return __ret;
            }
            let __self = self;
            let __ret: ::std::option::Option<
                ::zbus::fdo::Result<::zbus::zvariant::OwnedValue>,
            > = {
                match property_name {
                    "FailingProperty" => {
                        ::std::option::Option::Some(
                            __self
                                .failing_property()
                                .await
                                .map_err(|e| match e {
                                    zbus::Error::FDO(e) => *e,
                                    e => zbus::fdo::Error::Failed(e.to_string()),
                                })
                                .map(|e| {
                                    <::zbus::zvariant::Value as ::std::convert::From<
                                        _,
                                    >>::from(e)
                                        .to_owned()
                                }),
                        )
                    }
                    "GreeterName" => {
                        ::std::option::Option::Some(
                            ::std::result::Result::Ok(
                                ::std::convert::Into::into(
                                    <::zbus::zvariant::Value as ::std::convert::From<
                                        _,
                                    >>::from(__self.greeter_name().await),
                                ),
                            ),
                        )
                    }
                    _ => ::std::option::Option::None,
                }
            };
            #[allow(unreachable_code)] __ret
        })
    }
    #[allow(
        clippy::async_yields_async,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn get_all<'life0, 'async_trait>(
        &'life0 self,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = ::std::collections::HashMap<
                    ::std::string::String,
                    ::zbus::zvariant::OwnedValue,
                >,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret)
                = ::core::option::Option::None::<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        ::zbus::zvariant::OwnedValue,
                    >,
                > {
                return __ret;
            }
            let __self = self;
            let __ret: ::std::collections::HashMap<
                ::std::string::String,
                ::zbus::zvariant::OwnedValue,
            > = {
                let mut props: ::std::collections::HashMap<
                    ::std::string::String,
                    ::zbus::zvariant::OwnedValue,
                > = ::std::collections::HashMap::new();
                if let Ok(prop) = __self.failing_property().await {
                    props
                        .insert(
                            ::std::string::ToString::to_string("FailingProperty"),
                            ::std::convert::Into::into(
                                <::zbus::zvariant::Value as ::std::convert::From<
                                    _,
                                >>::from(prop),
                            ),
                        );
                }
                props
                    .insert(
                        ::std::string::ToString::to_string("GreeterName"),
                        ::std::convert::Into::into(
                            <::zbus::zvariant::Value as ::std::convert::From<
                                _,
                            >>::from(__self.greeter_name().await),
                        ),
                    );
                props
            };
            #[allow(unreachable_code)] __ret
        })
    }
    fn set<'call>(
        &'call self,
        property_name: &'call str,
        value: &'call ::zbus::zvariant::Value<'_>,
        signal_context: &'call ::zbus::SignalContext<'_>,
    ) -> ::zbus::DispatchResult<'call> {
        match property_name {
            "GreeterName" => ::zbus::DispatchResult::RequiresMut,
            _ => ::zbus::DispatchResult::NotFound,
        }
    }
    #[allow(
        clippy::async_yields_async,
        clippy::let_unit_value,
        clippy::no_effect_underscore_binding,
        clippy::shadow_same,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn set_mut<'life0, 'life1, 'life2, 'life3, 'life4, 'life5, 'async_trait>(
        &'life0 mut self,
        property_name: &'life1 str,
        value: &'life2 ::zbus::zvariant::Value<'life3>,
        signal_context: &'life4 ::zbus::SignalContext<'life5>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<
                Output = ::std::option::Option<::zbus::fdo::Result<()>>,
            > + ::core::marker::Send + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        'life3: 'async_trait,
        'life4: 'async_trait,
        'life5: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret)
                = ::core::option::Option::None::<
                    ::std::option::Option<::zbus::fdo::Result<()>>,
                > {
                return __ret;
            }
            let mut __self = self;
            let __ret: ::std::option::Option<::zbus::fdo::Result<()>> = {
                match property_name {
                    "GreeterName" => {
                        ::std::option::Option::Some({
                            let value = ::zbus::zvariant::Value::from(
                                zbus::zvariant::Value::to_owned(value),
                            );
                            match ::std::convert::TryInto::try_into(value) {
                                ::std::result::Result::Ok(val) => {
                                    match ::zbus::export::futures_util::future::FutureExt::map(
                                            __self.set_greeter_name(val),
                                            ::std::result::Result::Ok,
                                        )
                                        .await
                                    {
                                        ::std::result::Result::Ok(set_result) => {
                                            __self
                                                .greeter_name_changed(&signal_context)
                                                .await
                                                .map(|_| set_result)
                                                .map_err(Into::into)
                                        }
                                        e => e,
                                    }
                                }
                                ::std::result::Result::Err(e) => {
                                    ::std::result::Result::Err(
                                        ::std::convert::Into::into(
                                            ::zbus::Error::Variant(::std::convert::Into::into(e)),
                                        ),
                                    )
                                }
                            }
                        })
                    }
                    _ => ::std::option::Option::None,
                }
            };
            #[allow(unreachable_code)] __ret
        })
    }
    fn call<'call>(
        &'call self,
        s: &'call ::zbus::ObjectServer,
        c: &'call ::zbus::Connection,
        m: &'call ::zbus::Message,
        name: ::zbus::names::MemberName<'call>,
    ) -> ::zbus::DispatchResult<'call> {
        match name.as_str() {
            "SayHello" => {
                let future = async move {
                    let (name): (&str) = match m.body() {
                        ::std::result::Result::Ok(r) => r,
                        ::std::result::Result::Err(e) => {
                            let hdr = m.header()?;
                            let err = <::zbus::fdo::Error as ::std::convert::From<
                                _,
                            >>::from(e);
                            return c.reply_dbus_error(&hdr, err).await;
                        }
                    };
                    let reply = self.say_hello(name).await;
                    c.reply(m, &reply).await
                };
                ::zbus::DispatchResult::Async(
                    ::std::boxed::Box::pin(async move {
                        future.await.map(|_seq: u32| ())
                    }),
                )
            }
            "GoAway" => {
                let future = async move {
                    let ctxt = match m.path() {
                        ::std::option::Option::Some(p) => {
                            ::zbus::SignalContext::new(c, p)
                                .expect("Infallible conversion failed")
                        }
                        ::std::option::Option::None => {
                            let hdr = m.header()?;
                            let err = ::zbus::fdo::Error::UnknownObject(
                                "Path Required".into(),
                            );
                            return c.reply_dbus_error(&hdr, err).await;
                        }
                    };
                    let (): () = match m.body() {
                        ::std::result::Result::Ok(r) => r,
                        ::std::result::Result::Err(e) => {
                            let hdr = m.header()?;
                            let err = <::zbus::fdo::Error as ::std::convert::From<
                                _,
                            >>::from(e);
                            return c.reply_dbus_error(&hdr, err).await;
                        }
                    };
                    let reply = self.go_away(ctxt).await;
                    match reply {
                        ::std::result::Result::Ok(r) => c.reply(m, &r).await,
                        ::std::result::Result::Err(e) => {
                            let hdr = m.header()?;
                            c.reply_dbus_error(&hdr, e).await
                        }
                    }
                };
                ::zbus::DispatchResult::Async(
                    ::std::boxed::Box::pin(async move {
                        future.await.map(|_seq: u32| ())
                    }),
                )
            }
            _ => ::zbus::DispatchResult::NotFound,
        }
    }
    fn call_mut<'call>(
        &'call mut self,
        s: &'call ::zbus::ObjectServer,
        c: &'call ::zbus::Connection,
        m: &'call ::zbus::Message,
        name: ::zbus::names::MemberName<'call>,
    ) -> ::zbus::DispatchResult<'call> {
        match name.as_str() {
            _ => ::zbus::DispatchResult::NotFound,
        }
    }
    fn introspect_to_writer(&self, writer: &mut dyn ::std::fmt::Write, level: usize) {
        writer
            .write_fmt(
                ::core::fmt::Arguments::new_v1_formatted(
                    &["", "<interface name=\"", "\">\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&""),
                        ::core::fmt::ArgumentV1::new_display(
                            &<Self as ::zbus::Interface>::name(),
                        ),
                        ::core::fmt::ArgumentV1::from_usize(&level),
                    ],
                    &[
                        ::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 0u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Param(2usize),
                            },
                        },
                        ::core::fmt::rt::v1::Argument {
                            position: 1usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 0u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Implied,
                            },
                        },
                    ],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ),
            )
            .unwrap();
        {
            use ::zbus::zvariant::Type;
            let level = level + 2;
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "<method name=\"", "\">\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(&"SayHello"),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            {
                let level = level + 2;
                writer
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &["", "<arg name=\"", "\" type=\"", "\"", "/>\n"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&""),
                                ::core::fmt::ArgumentV1::new_display(&"name"),
                                ::core::fmt::ArgumentV1::new_display(&<&str>::signature()),
                                ::core::fmt::ArgumentV1::new_display(&" direction=\"in\""),
                                ::core::fmt::ArgumentV1::from_usize(&level),
                            ],
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Param(4usize),
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 2usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 3usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ),
                    )
                    .unwrap();
                writer
                    .write_fmt(
                        ::core::fmt::Arguments::new_v1_formatted(
                            &["", "<arg ", "type=\"", "\" direction=\"out\"/>\n"],
                            &[
                                ::core::fmt::ArgumentV1::new_display(&""),
                                ::core::fmt::ArgumentV1::new_display(&""),
                                ::core::fmt::ArgumentV1::new_display(
                                    &<String>::signature(),
                                ),
                                ::core::fmt::ArgumentV1::from_usize(&level),
                            ],
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Param(3usize),
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 2usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ),
                    )
                    .unwrap();
            }
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "</method>\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "<method name=\"", "\">\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(&"GoAway"),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            {
                let level = level + 2;
            }
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "</method>\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "<!--\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(
                                &" A signal; the implementation is provided by the macro.",
                            ),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", " -->\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "<signal name=\"", "\">\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(&"GreetedEveryone"),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            {
                let level = level + 2;
            }
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "</signal>\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "<!--\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(&" a failing property"),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", " -->\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &[
                            "",
                            "<property name=\"",
                            "\" type=\"",
                            "\" access=\"",
                            "\"/>\n",
                        ],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(&"FailingProperty"),
                            ::core::fmt::ArgumentV1::new_display(&<&str>::signature()),
                            ::core::fmt::ArgumentV1::new_display(&"read"),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(4usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 2usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 3usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "<!--\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(
                                &" A \"GreeterName\" property.",
                            ),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", " -->\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "<!--\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(
                                &" A setter for the \"GreeterName\" property.",
                            ),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer.write_fmt(::core::fmt::Arguments::new_v1(&["\n"], &[])).unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(
                                &" Additionally, a `greeter_name_changed` method has been generated for you if you need to",
                            ),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(
                                &" notify listeners that \"GreeterName\" was updated. It will be automatically called when",
                            ),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", "", "\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(
                                &" using this setter.  ",
                            ),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(2usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &["", " -->\n"],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(1usize),
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
            writer
                .write_fmt(
                    ::core::fmt::Arguments::new_v1_formatted(
                        &[
                            "",
                            "<property name=\"",
                            "\" type=\"",
                            "\" access=\"",
                            "\"/>\n",
                        ],
                        &[
                            ::core::fmt::ArgumentV1::new_display(&""),
                            ::core::fmt::ArgumentV1::new_display(&"GreeterName"),
                            ::core::fmt::ArgumentV1::new_display(&<&str>::signature()),
                            ::core::fmt::ArgumentV1::new_display(&"readwrite"),
                            ::core::fmt::ArgumentV1::from_usize(&level),
                        ],
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Param(4usize),
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 2usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 3usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ),
                )
                .unwrap();
        }
        writer
            .write_fmt(
                ::core::fmt::Arguments::new_v1_formatted(
                    &["", "</interface>\n"],
                    &[
                        ::core::fmt::ArgumentV1::new_display(&""),
                        ::core::fmt::ArgumentV1::from_usize(&level),
                    ],
                    &[
                        ::core::fmt::rt::v1::Argument {
                            position: 0usize,
                            format: ::core::fmt::rt::v1::FormatSpec {
                                fill: ' ',
                                align: ::core::fmt::rt::v1::Alignment::Unknown,
                                flags: 0u32,
                                precision: ::core::fmt::rt::v1::Count::Implied,
                                width: ::core::fmt::rt::v1::Count::Param(1usize),
                            },
                        },
                    ],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ),
            )
            .unwrap();
    }
}
fn main() -> Result<()> {
    async fn main() -> Result<()> {
        {
            let greeter = Greeter {
                name: "GreeterName".to_string(),
                done: event_listener::Event::new(),
            };
            let done_listener = greeter.done.listen();
            let connection = ConnectionBuilder::session()?
                .name("org.zbus.MyGreeter")?
                .serve_at("/org/zbus/MyGreeter", greeter)?
                .build()
                .await?;
            match &connection {
                tmp => {
                    {
                        ::std::io::_eprint(
                            ::core::fmt::Arguments::new_v1_formatted(
                                &["[", ":", "] ", " = ", "\n"],
                                &match (&"src/main.rs", &64u32, &"&connection", &&tmp) {
                                    args => {
                                        [
                                            ::core::fmt::ArgumentV1::new_display(args.0),
                                            ::core::fmt::ArgumentV1::new_display(args.1),
                                            ::core::fmt::ArgumentV1::new_display(args.2),
                                            ::core::fmt::ArgumentV1::new_debug(args.3),
                                        ]
                                    }
                                },
                                &[
                                    ::core::fmt::rt::v1::Argument {
                                        position: 0usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::core::fmt::rt::v1::Argument {
                                        position: 1usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::core::fmt::rt::v1::Argument {
                                        position: 2usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 0u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                    ::core::fmt::rt::v1::Argument {
                                        position: 3usize,
                                        format: ::core::fmt::rt::v1::FormatSpec {
                                            fill: ' ',
                                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                                            flags: 4u32,
                                            precision: ::core::fmt::rt::v1::Count::Implied,
                                            width: ::core::fmt::rt::v1::Count::Implied,
                                        },
                                    },
                                ],
                                unsafe { ::core::fmt::UnsafeArg::new() },
                            ),
                        );
                    };
                    tmp
                }
            };
            done_listener.wait();
            Ok(())
        }
    }
    async_std::task::block_on(async { main().await })
}
