// THIS FILE IS AUTOMATICALLY GENERATED BY SPACETIMEDB. EDITS TO THIS FILE
// WILL NOT BE SAVED. MODIFY TABLES IN RUST INSTEAD.

using System;
using ClientApi;
using Newtonsoft.Json.Linq;
using SpacetimeDB;

namespace SpacetimeDB.Types
{
	public static partial class Reducer
	{
		public delegate void PlayerSplitHandler(ReducerEvent reducerEvent);
		public static event PlayerSplitHandler OnPlayerSplitEvent;

		public static void PlayerSplit()
		{
			var _argArray = new object[] {};
			var _message = new SpacetimeDBClient.ReducerCallRequest {
				fn = "player_split",
				args = _argArray,
			};
			SpacetimeDBClient.instance.InternalCallReducer(Newtonsoft.Json.JsonConvert.SerializeObject(_message, _settings));
		}

		[ReducerCallback(FunctionName = "player_split")]
		public static bool OnPlayerSplit(ClientApi.Event dbEvent)
		{
			if(OnPlayerSplitEvent != null)
			{
				var args = ((ReducerEvent)dbEvent.FunctionCall.CallInfo).PlayerSplitArgs;
				OnPlayerSplitEvent((ReducerEvent)dbEvent.FunctionCall.CallInfo
				);
				return true;
			}
			return false;
		}

		[DeserializeEvent(FunctionName = "player_split")]
		public static void PlayerSplitDeserializeEventArgs(ClientApi.Event dbEvent)
		{
			var args = new PlayerSplitArgsStruct();
			var bsatnBytes = dbEvent.FunctionCall.ArgBytes;
			using var ms = new System.IO.MemoryStream();
			ms.SetLength(bsatnBytes.Length);
			bsatnBytes.CopyTo(ms.GetBuffer(), 0);
			ms.Position = 0;
			using var reader = new System.IO.BinaryReader(ms);
			dbEvent.FunctionCall.CallInfo = new ReducerEvent(ReducerType.PlayerSplit, "player_split", dbEvent.Timestamp, Identity.From(dbEvent.CallerIdentity.ToByteArray()), Address.From(dbEvent.CallerAddress.ToByteArray()), dbEvent.Message, dbEvent.Status, args);
		}
	}

	public partial class PlayerSplitArgsStruct
	{
	}

}
