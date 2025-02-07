// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN YOUR MODULE SOURCE CODE INSTEAD.

#nullable enable

using System;
using SpacetimeDB.ClientApi;
using System.Collections.Generic;
using System.Runtime.Serialization;

namespace SpacetimeDB.Types
{
    public sealed partial class RemoteReducers : RemoteBase
    {
        internal RemoteReducers(DbConnection conn, SetReducerFlags flags) : base(conn) => SetCallReducerFlags = flags;
        internal readonly SetReducerFlags SetCallReducerFlags;
    }

    public sealed partial class RemoteTables : RemoteTablesBase
    {
        public RemoteTables(DbConnection conn)
        {
            AddTable(Circle = new(conn));
            AddTable(CircleDecayTimer = new(conn));
            AddTable(CircleRecombineTimer = new(conn));
            AddTable(Config = new(conn));
            AddTable(ConsumeEntityTimer = new(conn));
            AddTable(Entity = new(conn));
            AddTable(Food = new(conn));
            AddTable(LoggedOutPlayer = new(conn));
            AddTable(MoveAllPlayersTimer = new(conn));
            AddTable(Player = new(conn));
            AddTable(SpawnFoodTimer = new(conn));
        }
    }

    public sealed partial class SetReducerFlags { }

    public sealed class EventContext : IEventContext
    {
        private readonly DbConnection conn;
        public readonly Event<Reducer> Event;

        public RemoteTables Db => conn.Db;
        public RemoteReducers Reducers => conn.Reducers;
        public SetReducerFlags SetReducerFlags => conn.SetReducerFlags;

        internal EventContext(DbConnection conn, Event<Reducer> Event)
        {
            this.conn = conn;
            this.Event = Event;
        }
    }

    public sealed class ReducerEventContext : IReducerEventContext
    {
        private readonly DbConnection conn;
        public readonly ReducerEvent<Reducer> Event;

        public RemoteTables Db => conn.Db;
        public RemoteReducers Reducers => conn.Reducers;
        public SetReducerFlags SetReducerFlags => conn.SetReducerFlags;

        internal ReducerEventContext(DbConnection conn, ReducerEvent<Reducer> reducerEvent)
        {
            this.conn = conn;
            Event = reducerEvent;
        }
    }

    public sealed class ErrorContext : IErrorContext
    {
        private readonly DbConnection conn;
        public readonly Exception Event;
        Exception IErrorContext.Event
        {
            get
            {
                return Event;
            }
        }

        public RemoteTables Db => conn.Db;
        public RemoteReducers Reducers => conn.Reducers;
        public SetReducerFlags SetReducerFlags => conn.SetReducerFlags;
        public Exception Error => Event;

        internal ErrorContext(DbConnection conn, Exception error)
        {
            this.conn = conn;
            Event = error;
        }
    }

    public sealed class SubscriptionEventContext : ISubscriptionEventContext
    {
        private readonly DbConnection conn;

        public RemoteTables Db => conn.Db;
        public RemoteReducers Reducers => conn.Reducers;
        public SetReducerFlags SetReducerFlags => conn.SetReducerFlags;

        internal SubscriptionEventContext(DbConnection conn)
        {
            this.conn = conn;
        }
    }

    public abstract partial class Reducer
    {
        private Reducer() { }

        public sealed class StdbNone : Reducer { }
    }

    public sealed class DbConnection : DbConnectionBase<DbConnection, RemoteTables, Reducer>
    {
        public override RemoteTables Db { get; }
        public readonly RemoteReducers Reducers;
        public readonly SetReducerFlags SetReducerFlags = new();

        public DbConnection()
        {
            Db = new(this);
            Reducers = new(this, SetReducerFlags);
        }

        protected override Reducer ToReducer(TransactionUpdate update)
        {
            var encodedArgs = update.ReducerCall.Args;
            return update.ReducerCall.ReducerName switch
            {
                "circle_decay" => BSATNHelpers.Decode<Reducer.CircleDecay>(encodedArgs),
                "circle_recombine" => BSATNHelpers.Decode<Reducer.CircleRecombine>(encodedArgs),
                "connect" => BSATNHelpers.Decode<Reducer.Connect>(encodedArgs),
                "consume_entity" => BSATNHelpers.Decode<Reducer.ConsumeEntity>(encodedArgs),
                "disconnect" => BSATNHelpers.Decode<Reducer.Disconnect>(encodedArgs),
                "enter_game" => BSATNHelpers.Decode<Reducer.EnterGame>(encodedArgs),
                "move_all_players" => BSATNHelpers.Decode<Reducer.MoveAllPlayers>(encodedArgs),
                "player_split" => BSATNHelpers.Decode<Reducer.PlayerSplit>(encodedArgs),
                "respawn" => BSATNHelpers.Decode<Reducer.Respawn>(encodedArgs),
                "spawn_food" => BSATNHelpers.Decode<Reducer.SpawnFood>(encodedArgs),
                "suicide" => BSATNHelpers.Decode<Reducer.Suicide>(encodedArgs),
                "update_player_input" => BSATNHelpers.Decode<Reducer.UpdatePlayerInput>(encodedArgs),
                "<none>" or "" => new Reducer.StdbNone(),
                var reducer => throw new ArgumentOutOfRangeException("Reducer", $"Unknown reducer {reducer}")
            };
        }

        protected override IEventContext ToEventContext(Event<Reducer> Event) =>
        new EventContext(this, Event);

        protected override IReducerEventContext ToReducerEventContext(ReducerEvent<Reducer> reducerEvent) =>
        new ReducerEventContext(this, reducerEvent);

        protected override ISubscriptionEventContext MakeSubscriptionEventContext() =>
        new SubscriptionEventContext(this);

        protected override IErrorContext ToErrorContext(Exception exception) =>
        new ErrorContext(this, exception);

        protected override bool Dispatch(IReducerEventContext context, Reducer reducer)
        {
            var eventContext = (ReducerEventContext)context;
            return reducer switch
            {
                Reducer.CircleDecay args => Reducers.InvokeCircleDecay(eventContext, args),
                Reducer.CircleRecombine args => Reducers.InvokeCircleRecombine(eventContext, args),
                Reducer.Connect args => Reducers.InvokeConnect(eventContext, args),
                Reducer.ConsumeEntity args => Reducers.InvokeConsumeEntity(eventContext, args),
                Reducer.Disconnect args => Reducers.InvokeDisconnect(eventContext, args),
                Reducer.EnterGame args => Reducers.InvokeEnterGame(eventContext, args),
                Reducer.MoveAllPlayers args => Reducers.InvokeMoveAllPlayers(eventContext, args),
                Reducer.PlayerSplit args => Reducers.InvokePlayerSplit(eventContext, args),
                Reducer.Respawn args => Reducers.InvokeRespawn(eventContext, args),
                Reducer.SpawnFood args => Reducers.InvokeSpawnFood(eventContext, args),
                Reducer.Suicide args => Reducers.InvokeSuicide(eventContext, args),
                Reducer.UpdatePlayerInput args => Reducers.InvokeUpdatePlayerInput(eventContext, args),
                Reducer.StdbNone => true,
                _ => throw new ArgumentOutOfRangeException("Reducer", $"Unknown reducer {reducer}")
            };
        }

        public SubscriptionBuilder<SubscriptionEventContext, ErrorContext> SubscriptionBuilder() => new(this);
    }
}
