// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]
pub struct PlayerSplit {}

impl __sdk::spacetime_module::InModule for PlayerSplit {
    type Module = super::RemoteModule;
}

pub struct PlayerSplitCallbackId(__sdk::callbacks::CallbackId);

#[allow(non_camel_case_types)]
/// Extension trait for access to the reducer `player_split`.
///
/// Implemented for [`super::RemoteReducers`].
pub trait player_split {
    /// Request that the remote module invoke the reducer `player_split` to run as soon as possible.
    ///
    /// This method returns immediately, and errors only if we are unable to send the request.
    /// The reducer will run asynchronously in the future,
    ///  and its status can be observed by listening for [`Self::on_player_split`] callbacks.
    fn player_split(&self) -> __anyhow::Result<()>;
    /// Register a callback to run whenever we are notified of an invocation of the reducer `player_split`.
    ///
    /// The [`super::EventContext`] passed to the `callback`
    /// will always have [`__sdk::Event::Reducer`] as its `event`,
    /// but it may or may not have terminated successfully and been committed.
    /// Callbacks should inspect the [`__sdk::ReducerEvent`] contained in the [`super::EventContext`]
    /// to determine the reducer's status.
    ///
    /// The returned [`PlayerSplitCallbackId`] can be passed to [`Self::remove_on_player_split`]
    /// to cancel the callback.
    fn on_player_split(
        &self,
        callback: impl FnMut(&super::EventContext) + Send + 'static,
    ) -> PlayerSplitCallbackId;
    /// Cancel a callback previously registered by [`Self::on_player_split`],
    /// causing it not to run in the future.
    fn remove_on_player_split(&self, callback: PlayerSplitCallbackId);
}

impl player_split for super::RemoteReducers {
    fn player_split(&self) -> __anyhow::Result<()> {
        self.imp.call_reducer("player_split", PlayerSplit {})
    }
    fn on_player_split(
        &self,
        mut callback: impl FnMut(&super::EventContext) + Send + 'static,
    ) -> PlayerSplitCallbackId {
        PlayerSplitCallbackId(self.imp.on_reducer::<PlayerSplit>(
            "player_split",
            Box::new(move |ctx: &super::EventContext, args: &PlayerSplit| callback(ctx)),
        ))
    }
    fn remove_on_player_split(&self, callback: PlayerSplitCallbackId) {
        self.imp
            .remove_on_reducer::<PlayerSplit>("player_split", callback.0)
    }
}

#[allow(non_camel_case_types)]
#[doc(hidden)]
/// Extension trait for setting the call-flags for the reducer `player_split`.
///
/// Implemented for [`super::SetReducerFlags`].
///
/// This type is currently unstable and may be removed without a major version bump.
pub trait set_flags_for_player_split {
    /// Set the call-reducer flags for the reducer `player_split` to `flags`.
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    fn player_split(&self, flags: __ws::CallReducerFlags);
}

impl set_flags_for_player_split for super::SetReducerFlags {
    fn player_split(&self, flags: __ws::CallReducerFlags) {
        self.imp.set_call_reducer_flags("player_split", flags);
    }
}
