# local_database.gd
class_name LocalDatabase extends Node # Or Node if signals are heavily used

# { "table_name_lower": { primary_key_value: RowResource } }
var _tables: Dictionary = {}
# { "table_name_lower": Script } - Row resource scripts
var _row_schemas: Dictionary = {}
# { "table_name_lower": StringName } - Cache for primary key field names
var _primary_key_cache: Dictionary = {}

# Signals (if needed, better if this is a Node or uses a SignalBus Autoload)
signal row_inserted(table_name: String, row: Resource)
signal row_updated(table_name: String, previous_row: Resource, row: Resource)
signal row_deleted(table_name: String, row: Resource) 
signal row_deleted_key(table_name: String, primary_key) 

func _init(row_schemas: Dictionary):
	self._row_schemas = row_schemas
	# Initialize _tables dictionary with known table names
	for table_name_lower in _row_schemas.keys():
		_tables[table_name_lower] = {}

# --- Primary Key Handling ---

# Finds and caches the primary key field name for a given schema
func _get_primary_key_field(table_name_lower: String) -> StringName:
	if _primary_key_cache.has(table_name_lower):
		return _primary_key_cache[table_name_lower]

	if not _row_schemas.has(table_name_lower):
		printerr("LocalDatabase: No schema found for table '", table_name_lower, "' to determine PK.")
		return &"" # Return empty StringName

	var schema: Script = _row_schemas[table_name_lower]
	var instance = schema.new() # Need instance for metadata/properties

	# 1. Check metadata (preferred)
	if instance and instance.has_meta("primary_key"):
		var pk_field : StringName = instance.get_meta("primary_key")
		_primary_key_cache[table_name_lower] = pk_field
		return pk_field

	# 2. Convention: Check for "identity" or "id" field
	var properties = schema.get_script_property_list()
	for prop in properties:
		if prop.usage & PROPERTY_USAGE_STORAGE:
			if prop.name == &"identity" or prop.name == &"id":
				_primary_key_cache[table_name_lower] = prop.name
				return prop.name
			# 3. Fallback: Assume first exported property (less reliable)
			# Uncomment if this is your desired convention
			# _primary_key_cache[table_name_lower] = prop.name
			# return prop.name

	printerr("LocalDatabase: Could not determine primary key for table '", table_name_lower, "'. Add metadata or use convention.")
	_primary_key_cache[table_name_lower] = &"" # Cache failure
	return &""


# --- Applying Updates ---

func apply_database_update(db_update: DatabaseUpdateData):
	if not db_update: return
	for table_update: TableUpdateData in db_update.tables:
		apply_table_update(table_update)

func apply_table_update(table_update: TableUpdateData):
	var table_name_lower := table_update.table_name.to_lower().replace("_","")
	if not _tables.has(table_name_lower):
		printerr("LocalDatabase: Received update for unknown table '", table_update.table_name, "'")
		# Optionally create the table entry: _tables[table_name_lower] = {}
		return

	var pk_field := _get_primary_key_field(table_name_lower)
	if pk_field == &"":
		printerr("LocalDatabase: Cannot apply update for table '", table_update.table_name, "' without primary key.")
		return

	var table_dict: Dictionary = _tables[table_name_lower]
	
	var inserted_pks: Array[Variant] = []

	# Process inserts/updates
	for inserted_row: Resource in table_update.inserts:
		var pk_value = inserted_row.get(pk_field)
		inserted_pks.append(pk_value)
		var is_update := table_dict.has(pk_value)

		# Store the previous row in case it's an update
		# Let's duplicate the resource just in case
		var prev_row: Resource = table_dict[pk_value].duplicate() if is_update else null
		table_dict[pk_value] = inserted_row # Add or overwrite

		if is_update:
			row_updated.emit(table_update.table_name, prev_row, inserted_row)
		else:
			row_inserted.emit(table_update.table_name, inserted_row)

	# Process deletes
	for deleted_row: Resource in table_update.deletes:
		var pk_value = deleted_row.get(pk_field)
		if table_dict.has(pk_value):
			if not inserted_pks.has(pk_value): ## Only emit deletes if the deleted PK wasn't re-inserted in the current transaction
				table_dict.erase(pk_value)
				row_deleted_key.emit(table_update.table_name, pk_value)
				row_deleted.emit(table_update.table_name, deleted_row)
		else:
			push_warning("LocalDatabase: Tried to delete row with PK '", pk_value, "' from table '", table_update.table_name, "' but it wasn't found.")


# --- Access Methods ---

func get_row(table_name: String, primary_key_value) -> Resource:
	var table_name_lower := table_name.to_lower().replace("_","")
	if _tables.has(table_name_lower):
		return _tables[table_name_lower].get(primary_key_value) # Returns null if not found
	return null
	
func get_all_rows(table_name: String) -> Array[Resource]:
	var table_name_lower := table_name.to_lower().replace("_","")
	if _tables.has(table_name_lower):
		var table_dict: Dictionary = _tables[table_name_lower]
		var values_array: Array = table_dict.values()
		var typed_result_array: Array[Resource] = []
		typed_result_array.assign(values_array)

		return typed_result_array
	else:
		return []
