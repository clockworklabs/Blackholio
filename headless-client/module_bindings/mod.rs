// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

#![allow(unused)]
use spacetimedb_sdk::{
    self as __sdk,
    anyhow::{self as __anyhow, Context as _},
    lib as __lib, sats as __sats, ws_messages as __ws,
};

pub mod circle_decay_reducer;
pub mod circle_decay_timer_table;
pub mod circle_decay_timer_type;
pub mod circle_table;
pub mod circle_type;
pub mod config_table;
pub mod config_type;
pub mod create_player_reducer;
pub mod entity_table;
pub mod entity_type;
pub mod food_table;
pub mod food_type;
pub mod identity_connected_reducer;
pub mod identity_disconnected_reducer;
pub mod init_reducer;
pub mod logged_out_circle_table;
pub mod logged_out_circle_type;
pub mod logged_out_player_table;
pub mod logged_out_player_type;
pub mod move_all_players_reducer;
pub mod move_all_players_timer_table;
pub mod move_all_players_timer_type;
pub mod player_split_reducer;
pub mod player_table;
pub mod player_type;
pub mod respawn_reducer;
pub mod spawn_food_reducer;
pub mod spawn_food_timer_table;
pub mod spawn_food_timer_type;
pub mod update_player_input_reducer;
pub mod vector_2_type;

pub use circle_decay_reducer::*;
pub use circle_decay_timer_table::*;
pub use circle_decay_timer_type::*;
pub use circle_table::*;
pub use circle_type::*;
pub use config_table::*;
pub use config_type::*;
pub use create_player_reducer::*;
pub use entity_table::*;
pub use entity_type::*;
pub use food_table::*;
pub use food_type::*;
pub use identity_connected_reducer::*;
pub use identity_disconnected_reducer::*;
pub use init_reducer::*;
pub use logged_out_circle_table::*;
pub use logged_out_circle_type::*;
pub use logged_out_player_table::*;
pub use logged_out_player_type::*;
pub use move_all_players_reducer::*;
pub use move_all_players_timer_table::*;
pub use move_all_players_timer_type::*;
pub use player_split_reducer::*;
pub use player_table::*;
pub use player_type::*;
pub use respawn_reducer::*;
pub use spawn_food_reducer::*;
pub use spawn_food_timer_table::*;
pub use spawn_food_timer_type::*;
pub use update_player_input_reducer::*;
pub use vector_2_type::*;

#[derive(__lib::ser::Serialize, __lib::de::Deserialize, Clone, PartialEq, Debug)]
#[sats(crate = __lib)]

/// One of the reducers defined by this module.
///
/// Contained within a [`__sdk::ReducerEvent`] in [`EventContext`]s for reducer events
/// to indicate which reducer caused the event.

pub enum Reducer {
    IdentityConnected(identity_connected_reducer::IdentityConnected),
    IdentityDisconnected(identity_disconnected_reducer::IdentityDisconnected),
    Init(init_reducer::Init),
    CircleDecay(circle_decay_reducer::CircleDecay),
    CreatePlayer(create_player_reducer::CreatePlayer),
    MoveAllPlayers(move_all_players_reducer::MoveAllPlayers),
    PlayerSplit(player_split_reducer::PlayerSplit),
    Respawn(respawn_reducer::Respawn),
    SpawnFood(spawn_food_reducer::SpawnFood),
    UpdatePlayerInput(update_player_input_reducer::UpdatePlayerInput),
}

impl __sdk::spacetime_module::InModule for Reducer {
    type Module = RemoteModule;
}

impl __sdk::spacetime_module::Reducer for Reducer {
    fn reducer_name(&self) -> &'static str {
        match self {
            Reducer::IdentityConnected(_) => "__identity_connected__",
            Reducer::IdentityDisconnected(_) => "__identity_disconnected__",
            Reducer::Init(_) => "__init__",
            Reducer::CircleDecay(_) => "circle_decay",
            Reducer::CreatePlayer(_) => "create_player",
            Reducer::MoveAllPlayers(_) => "move_all_players",
            Reducer::PlayerSplit(_) => "player_split",
            Reducer::Respawn(_) => "respawn",
            Reducer::SpawnFood(_) => "spawn_food",
            Reducer::UpdatePlayerInput(_) => "update_player_input",
        }
    }
    fn reducer_args(&self) -> &dyn std::any::Any {
        match self {
            Reducer::IdentityConnected(args) => args,
            Reducer::IdentityDisconnected(args) => args,
            Reducer::Init(args) => args,
            Reducer::CircleDecay(args) => args,
            Reducer::CreatePlayer(args) => args,
            Reducer::MoveAllPlayers(args) => args,
            Reducer::PlayerSplit(args) => args,
            Reducer::Respawn(args) => args,
            Reducer::SpawnFood(args) => args,
            Reducer::UpdatePlayerInput(args) => args,
        }
    }
}
impl TryFrom<__ws::ReducerCallInfo<__ws::BsatnFormat>> for Reducer {
    type Error = __anyhow::Error;
    fn try_from(value: __ws::ReducerCallInfo<__ws::BsatnFormat>) -> __anyhow::Result<Self> {
        match &value.reducer_name[..] {
            "__identity_connected__" => Ok(Reducer::IdentityConnected(
                __sdk::spacetime_module::parse_reducer_args("__identity_connected__", &value.args)?,
            )),
            "__identity_disconnected__" => Ok(Reducer::IdentityDisconnected(
                __sdk::spacetime_module::parse_reducer_args(
                    "__identity_disconnected__",
                    &value.args,
                )?,
            )),
            "__init__" => Ok(Reducer::Init(__sdk::spacetime_module::parse_reducer_args(
                "__init__",
                &value.args,
            )?)),
            "circle_decay" => Ok(Reducer::CircleDecay(
                __sdk::spacetime_module::parse_reducer_args("circle_decay", &value.args)?,
            )),
            "create_player" => Ok(Reducer::CreatePlayer(
                __sdk::spacetime_module::parse_reducer_args("create_player", &value.args)?,
            )),
            "move_all_players" => Ok(Reducer::MoveAllPlayers(
                __sdk::spacetime_module::parse_reducer_args("move_all_players", &value.args)?,
            )),
            "player_split" => Ok(Reducer::PlayerSplit(
                __sdk::spacetime_module::parse_reducer_args("player_split", &value.args)?,
            )),
            "respawn" => Ok(Reducer::Respawn(
                __sdk::spacetime_module::parse_reducer_args("respawn", &value.args)?,
            )),
            "spawn_food" => Ok(Reducer::SpawnFood(
                __sdk::spacetime_module::parse_reducer_args("spawn_food", &value.args)?,
            )),
            "update_player_input" => Ok(Reducer::UpdatePlayerInput(
                __sdk::spacetime_module::parse_reducer_args("update_player_input", &value.args)?,
            )),
            _ => Err(__anyhow::anyhow!(
                "Unknown reducer {:?}",
                value.reducer_name
            )),
        }
    }
}

#[derive(Default)]
#[allow(non_snake_case)]
#[doc(hidden)]
pub struct DbUpdate {
    circle: __sdk::spacetime_module::TableUpdate<Circle>,
    circle_decay_timer: __sdk::spacetime_module::TableUpdate<CircleDecayTimer>,
    config: __sdk::spacetime_module::TableUpdate<Config>,
    entity: __sdk::spacetime_module::TableUpdate<Entity>,
    food: __sdk::spacetime_module::TableUpdate<Food>,
    logged_out_circle: __sdk::spacetime_module::TableUpdate<LoggedOutCircle>,
    logged_out_player: __sdk::spacetime_module::TableUpdate<LoggedOutPlayer>,
    move_all_players_timer: __sdk::spacetime_module::TableUpdate<MoveAllPlayersTimer>,
    player: __sdk::spacetime_module::TableUpdate<Player>,
    spawn_food_timer: __sdk::spacetime_module::TableUpdate<SpawnFoodTimer>,
}

impl TryFrom<__ws::DatabaseUpdate<__ws::BsatnFormat>> for DbUpdate {
    type Error = __anyhow::Error;
    fn try_from(raw: __ws::DatabaseUpdate<__ws::BsatnFormat>) -> Result<Self, Self::Error> {
        let mut db_update = DbUpdate::default();
        for table_update in raw.tables {
            match &table_update.table_name[..] {
                "circle" => db_update.circle = circle_table::parse_table_update(table_update)?,
                "circle_decay_timer" => {
                    db_update.circle_decay_timer =
                        circle_decay_timer_table::parse_table_update(table_update)?
                }
                "config" => db_update.config = config_table::parse_table_update(table_update)?,
                "entity" => db_update.entity = entity_table::parse_table_update(table_update)?,
                "food" => db_update.food = food_table::parse_table_update(table_update)?,
                "logged_out_circle" => {
                    db_update.logged_out_circle =
                        logged_out_circle_table::parse_table_update(table_update)?
                }
                "logged_out_player" => {
                    db_update.logged_out_player =
                        logged_out_player_table::parse_table_update(table_update)?
                }
                "move_all_players_timer" => {
                    db_update.move_all_players_timer =
                        move_all_players_timer_table::parse_table_update(table_update)?
                }
                "player" => db_update.player = player_table::parse_table_update(table_update)?,
                "spawn_food_timer" => {
                    db_update.spawn_food_timer =
                        spawn_food_timer_table::parse_table_update(table_update)?
                }

                unknown => __anyhow::bail!("Unknown table {unknown:?} in DatabaseUpdate"),
            }
        }
        Ok(db_update)
    }
}

impl __sdk::spacetime_module::InModule for DbUpdate {
    type Module = RemoteModule;
}

impl __sdk::spacetime_module::DbUpdate for DbUpdate {
    fn apply_to_client_cache(&self, cache: &mut __sdk::client_cache::ClientCache<RemoteModule>) {
        cache.apply_diff_to_table::<Circle>("circle", &self.circle);
        cache.apply_diff_to_table::<CircleDecayTimer>(
            "circle_decay_timer",
            &self.circle_decay_timer,
        );
        cache.apply_diff_to_table::<Config>("config", &self.config);
        cache.apply_diff_to_table::<Entity>("entity", &self.entity);
        cache.apply_diff_to_table::<Food>("food", &self.food);
        cache.apply_diff_to_table::<LoggedOutCircle>("logged_out_circle", &self.logged_out_circle);
        cache.apply_diff_to_table::<LoggedOutPlayer>("logged_out_player", &self.logged_out_player);
        cache.apply_diff_to_table::<MoveAllPlayersTimer>(
            "move_all_players_timer",
            &self.move_all_players_timer,
        );
        cache.apply_diff_to_table::<Player>("player", &self.player);
        cache.apply_diff_to_table::<SpawnFoodTimer>("spawn_food_timer", &self.spawn_food_timer);
    }
    fn invoke_row_callbacks(
        &self,
        event: &EventContext,
        callbacks: &mut __sdk::callbacks::DbCallbacks<RemoteModule>,
    ) {
        callbacks.invoke_table_row_callbacks::<Circle>("circle", &self.circle, event);
        callbacks.invoke_table_row_callbacks::<CircleDecayTimer>(
            "circle_decay_timer",
            &self.circle_decay_timer,
            event,
        );
        callbacks.invoke_table_row_callbacks::<Config>("config", &self.config, event);
        callbacks.invoke_table_row_callbacks::<Entity>("entity", &self.entity, event);
        callbacks.invoke_table_row_callbacks::<Food>("food", &self.food, event);
        callbacks.invoke_table_row_callbacks::<LoggedOutCircle>(
            "logged_out_circle",
            &self.logged_out_circle,
            event,
        );
        callbacks.invoke_table_row_callbacks::<LoggedOutPlayer>(
            "logged_out_player",
            &self.logged_out_player,
            event,
        );
        callbacks.invoke_table_row_callbacks::<MoveAllPlayersTimer>(
            "move_all_players_timer",
            &self.move_all_players_timer,
            event,
        );
        callbacks.invoke_table_row_callbacks::<Player>("player", &self.player, event);
        callbacks.invoke_table_row_callbacks::<SpawnFoodTimer>(
            "spawn_food_timer",
            &self.spawn_food_timer,
            event,
        );
    }
}

#[doc(hidden)]
pub struct RemoteModule;

impl __sdk::spacetime_module::InModule for RemoteModule {
    type Module = Self;
}

impl __sdk::spacetime_module::SpacetimeModule for RemoteModule {
    type DbConnection = DbConnection;
    type EventContext = EventContext;
    type Reducer = Reducer;
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;
    type DbUpdate = DbUpdate;
    type SubscriptionHandle = SubscriptionHandle;
}

/// The `reducers` field of [`EventContext`] and [`DbConnection`],
/// with methods provided by extension traits for each reducer defined by the module.
pub struct RemoteReducers {
    imp: __sdk::db_connection::DbContextImpl<RemoteModule>,
}

impl __sdk::spacetime_module::InModule for RemoteReducers {
    type Module = RemoteModule;
}

#[doc(hidden)]
/// The `set_reducer_flags` field of [`DbConnection`],
/// with methods provided by extension traits for each reducer defined by the module.
/// Each method sets the flags for the reducer with the same name.
///
/// This type is currently unstable and may be removed without a major version bump.
pub struct SetReducerFlags {
    imp: __sdk::db_connection::DbContextImpl<RemoteModule>,
}

impl __sdk::spacetime_module::InModule for SetReducerFlags {
    type Module = RemoteModule;
}

/// The `db` field of [`EventContext`] and [`DbConnection`],
/// with methods provided by extension traits for each table defined by the module.
pub struct RemoteTables {
    imp: __sdk::db_connection::DbContextImpl<RemoteModule>,
}

impl __sdk::spacetime_module::InModule for RemoteTables {
    type Module = RemoteModule;
}

/// A connection to a remote module, including a materialized view of a subset of the database.
///
/// Connect to a remote module by calling [`DbConnection::builder`]
/// and using the [`__sdk::DbConnectionBuilder`] builder-pattern constructor.
///
/// You must explicitly advance the connection by calling any one of:
///
/// - [`DbConnection::frame_tick`].
/// - [`DbConnection::run_threaded`].
/// - [`DbConnection::run_async`].
/// - [`DbConnection::advance_one_message`].
/// - [`DbConnection::advance_one_message_blocking`].
/// - [`DbConnection::advance_one_message_async`].
///
/// Which of these methods you should call depends on the specific needs of your application,
/// but you must call one of them, or else the connection will never progress.
pub struct DbConnection {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    #[doc(hidden)]
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,

    imp: __sdk::db_connection::DbContextImpl<RemoteModule>,
}

impl __sdk::spacetime_module::InModule for DbConnection {
    type Module = RemoteModule;
}

impl __sdk::db_context::DbContext for DbConnection {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> __anyhow::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::subscription::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::subscription::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn address(&self) -> __sdk::Address {
        self.imp.address()
    }
}

impl DbConnection {
    /// Builder-pattern constructor for a connection to a remote module.
    ///
    /// See [`__sdk::DbConnectionBuilder`] for required and optional configuration for the new connection.
    pub fn builder() -> __sdk::DbConnectionBuilder<RemoteModule> {
        __sdk::db_connection::DbConnectionBuilder::new()
    }

    /// If any WebSocket messages are waiting, process one of them.
    ///
    /// Returns `true` if a message was processed, or `false` if the queue is empty.
    /// Callers should invoke this message in a loop until it returns `false`
    /// or for as much time is available to process messages.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::frame_tick`] each frame
    /// to fully exhaust the queue whenever time is available.
    pub fn advance_one_message(&self) -> __anyhow::Result<bool> {
        self.imp.advance_one_message()
    }

    /// Process one WebSocket message, potentially blocking the current thread until one is received.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::run_threaded`] to spawn a thread
    /// which advances the connection automatically.
    pub fn advance_one_message_blocking(&self) -> __anyhow::Result<()> {
        self.imp.advance_one_message_blocking()
    }

    /// Process one WebSocket message, `await`ing until one is received.
    ///
    /// Returns an error if the connection is disconnected.
    /// If the disconnection in question was normal,
    ///  i.e. the result of a call to [`__sdk::DbContext::disconnect`],
    /// the returned error will be downcastable to [`__sdk::DisconnectedError`].
    ///
    /// This is a low-level primitive exposed for power users who need significant control over scheduling.
    /// Most applications should call [`Self::run_async`] to run an `async` loop
    /// which advances the connection when polled.
    pub async fn advance_one_message_async(&self) -> __anyhow::Result<()> {
        self.imp.advance_one_message_async().await
    }

    /// Process all WebSocket messages waiting in the queue,
    /// then return without `await`ing or blocking the current thread.
    pub fn frame_tick(&self) -> __anyhow::Result<()> {
        self.imp.frame_tick()
    }

    /// Spawn a thread which processes WebSocket messages as they are received.
    pub fn run_threaded(&self) -> std::thread::JoinHandle<()> {
        self.imp.run_threaded()
    }

    /// Run an `async` loop which processes WebSocket messages when polled.
    pub async fn run_async(&self) -> __anyhow::Result<()> {
        self.imp.run_async().await
    }
}

impl __sdk::spacetime_module::DbConnection for DbConnection {
    fn new(imp: __sdk::db_connection::DbContextImpl<RemoteModule>) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            imp,
        }
    }
}

/// A [`DbConnection`] augmented with an [`__sdk::Event`],
/// passed to various callbacks invoked by the SDK.
pub struct EventContext {
    /// Access to tables defined by the module via extension traits implemented for [`RemoteTables`].
    pub db: RemoteTables,
    /// Access to reducers defined by the module via extension traits implemented for [`RemoteReducers`].
    pub reducers: RemoteReducers,
    /// Access to setting the call-flags of each reducer defined for each reducer defined by the module
    /// via extension traits implemented for [`SetReducerFlags`].
    ///
    /// This type is currently unstable and may be removed without a major version bump.
    pub set_reducer_flags: SetReducerFlags,
    /// The event which caused these callbacks to run.
    pub event: __sdk::event::Event<Reducer>,
    imp: __sdk::db_connection::DbContextImpl<RemoteModule>,
}

impl __sdk::spacetime_module::InModule for EventContext {
    type Module = RemoteModule;
}

impl __sdk::db_context::DbContext for EventContext {
    type DbView = RemoteTables;
    type Reducers = RemoteReducers;
    type SetReducerFlags = SetReducerFlags;

    fn db(&self) -> &Self::DbView {
        &self.db
    }
    fn reducers(&self) -> &Self::Reducers {
        &self.reducers
    }
    fn set_reducer_flags(&self) -> &Self::SetReducerFlags {
        &self.set_reducer_flags
    }

    fn is_active(&self) -> bool {
        self.imp.is_active()
    }

    fn disconnect(&self) -> spacetimedb_sdk::anyhow::Result<()> {
        self.imp.disconnect()
    }

    type SubscriptionBuilder = __sdk::subscription::SubscriptionBuilder<RemoteModule>;

    fn subscription_builder(&self) -> Self::SubscriptionBuilder {
        __sdk::subscription::SubscriptionBuilder::new(&self.imp)
    }

    fn try_identity(&self) -> Option<__sdk::Identity> {
        self.imp.try_identity()
    }
    fn address(&self) -> __sdk::Address {
        self.imp.address()
    }
}

impl __sdk::spacetime_module::EventContext for EventContext {
    fn event(&self) -> &__sdk::event::Event<Reducer> {
        &self.event
    }
    fn new(
        imp: __sdk::db_connection::DbContextImpl<RemoteModule>,
        event: __sdk::event::Event<Reducer>,
    ) -> Self {
        Self {
            db: RemoteTables { imp: imp.clone() },
            reducers: RemoteReducers { imp: imp.clone() },
            set_reducer_flags: SetReducerFlags { imp: imp.clone() },
            event,
            imp,
        }
    }
}

/// A handle on a subscribed query.
// TODO: Document this better after implementing the new subscription API.
pub struct SubscriptionHandle {
    imp: __sdk::subscription::SubscriptionHandleImpl<RemoteModule>,
}

impl __sdk::spacetime_module::InModule for SubscriptionHandle {
    type Module = RemoteModule;
}

impl __sdk::spacetime_module::SubscriptionHandle for SubscriptionHandle {
    fn new(imp: __sdk::subscription::SubscriptionHandleImpl<RemoteModule>) -> Self {
        Self { imp }
    }
}

/// Alias trait for a [`__sdk::DbContext`] connected to this module,
/// with that trait's associated types bounded to this module's concrete types.
///
/// Users can use this trait as a boundary on definitions which should accept
/// either a [`DbConnection`] or an [`EventContext`] and operate on either.
pub trait RemoteDbContext:
    __sdk::DbContext<
    DbView = RemoteTables,
    Reducers = RemoteReducers,
    SetReducerFlags = SetReducerFlags,
    SubscriptionBuilder = __sdk::subscription::SubscriptionBuilder<RemoteModule>,
>
{
}
impl<
        Ctx: __sdk::DbContext<
            DbView = RemoteTables,
            Reducers = RemoteReducers,
            SetReducerFlags = SetReducerFlags,
            SubscriptionBuilder = __sdk::subscription::SubscriptionBuilder<RemoteModule>,
        >,
    > RemoteDbContext for Ctx
{
}
