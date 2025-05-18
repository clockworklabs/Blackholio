extends Node2D

func _ready():
	# Connect to signals BEFORE connecting to the DB
	SpacetimeDB.connected.connect(_on_spacetimedb_connected)
	SpacetimeDB.disconnected.connect(_on_spacetimedb_disconnected)
	SpacetimeDB.connection_error.connect(_on_spacetimedb_connection_error)
	SpacetimeDB.identity_received.connect(_on_spacetimedb_identity_received)
	SpacetimeDB.database_initialized.connect(_on_spacetimedb_database_initialized)
	SpacetimeDB.transaction_update_received.connect(_on_transaction_update) # For reducer results

	var options = SpacetimeDBConnectionOptions.new()
	options.compression = SpacetimeDBConnection.CompressionPreference.NONE
	options.one_time_token = true
	options.debug_mode = false
	options.inbound_buffer_size = 1024 * 1024 * 2 # 2MB
	options.outbound_buffer_size = 1024 * 1024 * 2 # 2MB

	SpacetimeDB.connect_db(
		"http://127.0.0.1:3000", # Base HTTP URL
		"blackholio", # Module Name
		options
	)
	# ------------------------------------

func _on_spacetimedb_connected():
	print("Game: Connected to SpacetimeDB!")
	# Good place to subscribe to initial data
	var queries = ["SELECT * FROM player"]
	var req_id = SpacetimeDB.subscribe(queries)
	if req_id < 0: printerr("Subscription failed!")
	
	# TODO: achieve the following logic in gdscript
	#Debug.Log("Connected.");
	#AuthToken.SaveToken(token);
	#LocalIdentity = identity;
#
	#conn.Db.Circle.OnInsert += CircleOnInsert;
	#conn.Db.Entity.OnUpdate += EntityOnUpdate;
	#conn.Db.Entity.OnDelete += EntityOnDelete;
	#conn.Db.Food.OnInsert += FoodOnInsert;
	#conn.Db.Player.OnInsert += PlayerOnInsert;
	#conn.Db.Player.OnDelete += PlayerOnDelete;
#
	#OnConnected?.Invoke();
#
	#// Request all tables
	#Conn.SubscriptionBuilder()
		#.OnApplied(HandleSubscriptionApplied)
		#.SubscribeToAllTables();

  
func _on_spacetimedb_identity_received(identity_token: IdentityTokenData):
	print("Game: My Identity: 0x%s" % identity_token.identity.hex_encode())
	# Store identity if needed, e.g., var my_identity = identity_token.identity

func _on_spacetimedb_database_initialized():
	print("Game: Local database cache initialized.")
	# Safe to query the local DB for initially subscribed data
	var db = SpacetimeDB.get_local_database()
	var initial_players = db.get_all_rows("player")
	print("Initial players found: %d" % initial_players.size())
	# ... setup initial game state ...

func _on_spacetimedb_disconnected():
	print("Game: Disconnected.")

func _on_spacetimedb_connection_error(code, reason):
	printerr("Game: Connection Error (Code: %d): %s" % [code, reason])

func _on_transaction_update(update: TransactionUpdateData):
	# Handle results/errors from reducer calls
	if update.status.status_type == UpdateStatusData.StatusType.FAILED:
		printerr("Reducer call (ReqID: %d) failed: %s" % [update.reducer_call.request_id, update.status.failure_message])
	elif update.status.status_type == UpdateStatusData.StatusType.COMMITTED:
		print("Reducer call (ReqID: %d) committed." % update.reducer_call.request_id)
		# Optionally inspect update.status.committed_update for DB changes
