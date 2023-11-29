// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;
using System.Collections.Generic;
using SpacetimeDB;

namespace SpacetimeDB.Types
{
	[Newtonsoft.Json.JsonObject(Newtonsoft.Json.MemberSerialization.OptIn)]
	public partial class Player : IDatabaseTable
	{
		[Newtonsoft.Json.JsonProperty("identity")]
		public SpacetimeDB.Identity Identity;
		[Newtonsoft.Json.JsonProperty("player_id")]
		public uint PlayerId;
		[Newtonsoft.Json.JsonProperty("name")]
		public string Name;

		private static Dictionary<SpacetimeDB.Identity, Player> Identity_Index = new Dictionary<SpacetimeDB.Identity, Player>(16);
		private static Dictionary<uint, Player> PlayerId_Index = new Dictionary<uint, Player>(16);

		private static void InternalOnValueInserted(object insertedValue)
		{
			var val = (Player)insertedValue;
			Identity_Index[val.Identity] = val;
			PlayerId_Index[val.PlayerId] = val;
		}

		private static void InternalOnValueDeleted(object deletedValue)
		{
			var val = (Player)deletedValue;
			Identity_Index.Remove(val.Identity);
			PlayerId_Index.Remove(val.PlayerId);
		}

		public static SpacetimeDB.SATS.AlgebraicType GetAlgebraicType()
		{
			return SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("identity", SpacetimeDB.SATS.AlgebraicType.CreateProductType(new SpacetimeDB.SATS.ProductTypeElement[]
			{
				new SpacetimeDB.SATS.ProductTypeElement("__identity_bytes", SpacetimeDB.SATS.AlgebraicType.CreateArrayType(SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U8))),
			})),
				new SpacetimeDB.SATS.ProductTypeElement("player_id", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.U32)),
				new SpacetimeDB.SATS.ProductTypeElement("name", SpacetimeDB.SATS.AlgebraicType.CreatePrimitiveType(SpacetimeDB.SATS.BuiltinType.Type.String)),
			});
		}

		public static explicit operator Player(SpacetimeDB.SATS.AlgebraicValue value)
		{
			if (value == null) {
				return null;
			}
			var productValue = value.AsProductValue();
			return new Player
			{
				Identity = SpacetimeDB.Identity.From(productValue.elements[0].AsProductValue().elements[0].AsBytes()),
				PlayerId = productValue.elements[1].AsU32(),
				Name = productValue.elements[2].AsString(),
			};
		}

		public static System.Collections.Generic.IEnumerable<Player> Iter()
		{
			foreach(var entry in SpacetimeDBClient.clientDB.GetEntries("Player"))
			{
				yield return (Player)entry.Item2;
			}
		}
		public static int Count()
		{
			return SpacetimeDBClient.clientDB.Count("Player");
		}
		public static Player FilterByIdentity(SpacetimeDB.Identity value)
		{
			Identity_Index.TryGetValue(value, out var r);
			return r;
		}

		public static Player FilterByPlayerId(uint value)
		{
			PlayerId_Index.TryGetValue(value, out var r);
			return r;
		}

		public static System.Collections.Generic.IEnumerable<Player> FilterByName(string value)
		{
			foreach(var entry in SpacetimeDBClient.clientDB.GetEntries("Player"))
			{
				var productValue = entry.Item1.AsProductValue();
				var compareValue = (string)productValue.elements[2].AsString();
				if (compareValue == value) {
					yield return (Player)entry.Item2;
				}
			}
		}

		public static bool ComparePrimaryKey(SpacetimeDB.SATS.AlgebraicType t, SpacetimeDB.SATS.AlgebraicValue v1, SpacetimeDB.SATS.AlgebraicValue v2)
		{
			var primaryColumnValue1 = v1.AsProductValue().elements[0];
			var primaryColumnValue2 = v2.AsProductValue().elements[0];
			return SpacetimeDB.SATS.AlgebraicValue.Compare(t.product.elements[0].algebraicType, primaryColumnValue1, primaryColumnValue2);
		}

		public static SpacetimeDB.SATS.AlgebraicValue GetPrimaryKeyValue(SpacetimeDB.SATS.AlgebraicValue v)
		{
			return v.AsProductValue().elements[0];
		}

		public static SpacetimeDB.SATS.AlgebraicType GetPrimaryKeyType(SpacetimeDB.SATS.AlgebraicType t)
		{
			return t.product.elements[0].algebraicType;
		}

		public delegate void InsertEventHandler(Player insertedValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void UpdateEventHandler(Player oldValue, Player newValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void DeleteEventHandler(Player deletedValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public delegate void RowUpdateEventHandler(SpacetimeDBClient.TableOp op, Player oldValue, Player newValue, SpacetimeDB.Types.ReducerEvent dbEvent);
		public static event InsertEventHandler OnInsert;
		public static event UpdateEventHandler OnUpdate;
		public static event DeleteEventHandler OnBeforeDelete;
		public static event DeleteEventHandler OnDelete;
		public static event RowUpdateEventHandler OnRowUpdate;

		public static void OnInsertEvent(object newValue, ClientApi.Event dbEvent)
		{
			OnInsert?.Invoke((Player)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnUpdateEvent(object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnUpdate?.Invoke((Player)oldValue,(Player)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnBeforeDeleteEvent(object oldValue, ClientApi.Event dbEvent)
		{
			OnBeforeDelete?.Invoke((Player)oldValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnDeleteEvent(object oldValue, ClientApi.Event dbEvent)
		{
			OnDelete?.Invoke((Player)oldValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}

		public static void OnRowUpdateEvent(SpacetimeDBClient.TableOp op, object oldValue, object newValue, ClientApi.Event dbEvent)
		{
			OnRowUpdate?.Invoke(op, (Player)oldValue,(Player)newValue,(ReducerEvent)dbEvent?.FunctionCall.CallInfo);
		}
	}
}
