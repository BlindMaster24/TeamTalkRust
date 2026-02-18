//! Macro definitions for TeamTalk bots.

/// Macro for creating an event dispatcher with a concise syntax.
///
/// # Example
/// ```rust
/// let dispatcher = teamtalk_events! {
///     on_user_joined_data(user, _ctx) => {
///         println!("User joined: {}", user.nickname);
///         DispatchFlow::Continue
///     }
/// };
/// ```
#[macro_export]
macro_rules! teamtalk_events {
    (
        $($name:ident($($arg:pat),*) => $body:block)*
    ) => {
        {
            use $crate::dispatch::{Dispatcher, DispatchFlow};
            let mut dispatcher = Dispatcher::new();
            $(
                dispatcher = dispatcher.$name(move |$($arg),*| $body);
            )*
            dispatcher
        }
    };
}
