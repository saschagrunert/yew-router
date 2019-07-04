//! Yew routing extension

#![deny(missing_docs)]

mod router;
pub use crate::router::{Request, Route, RouterAgent};

#[macro_export]
/// Convinience macro for route creation
macro_rules! routes {
    ($($x:tt => $y:expr,)*) => (
        #[derive(Debug, PartialEq)]
        /// Possible child components
        pub enum RouterTarget {
            $($x,)*
        }

        /// Convert a RouterTarget into a Route
        impl<T> ::std::convert::Into<::yew_router::Route<T>> for RouterTarget where T: Default {
            fn into(self) -> ::yew_router::Route<T> {
                ::yew_router::Route {
                    fragment: Some(
                        match self {
                            $(RouterTarget::$x => $y,)*
                        }.into(),
                    ),
                    ..Default::default()
                }
            }
        }

        /// Convert a Route into a RouterTarget
        impl ::std::convert::Into<RouterTarget> for ::yew_router::Route<()> {
            fn into(self) -> RouterTarget {
                match self.fragment {
                    Some(f) => match f.as_str() {
                        $($y => RouterTarget::$x,)*
                        _ => RouterTarget::Error,
                    },
                    _ => RouterTarget::Error,
                }
            }
        }
    )
}
