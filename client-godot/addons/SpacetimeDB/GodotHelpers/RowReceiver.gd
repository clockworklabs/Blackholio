@tool
@icon("res://addons/SpacetimeDB/Icons/node_icon.svg")
extends Node
class_name RowReceiver

@export var table_to_receive: ModuleTable : set=on_set;

signal insert(row: ModuleTable)
signal update(prev: ModuleTable, row: ModuleTable)
signal delete(row: ModuleTable)

func on_set(schema: ModuleTable):
	if schema == null:
		name = "Receiver [EMPTY]"
		table_to_receive = null
		return
	var global_name: String = schema.get_script().get_global_name().replace("_gd", "")
	if global_name == "ModuleTable": 
		push_error("ModuleTable is the base class for tables, not a reciever table. Selection is not changed.")
		return
	if not global_name.contains("Table"):
		push_error("{0} is a base struct, not a reciever table. Selection is not changed.".format(
			[global_name]))
		return
	name = "Receiver [%s]" % global_name
	table_to_receive = schema;

func _ready() -> void:
	if Engine.is_editor_hint():
		return;
		
	SpacetimeDB.row_inserted.connect(_on_insert)
	SpacetimeDB.row_updated.connect(_on_update)
	SpacetimeDB.row_deleted.connect(_on_delete)

	if not table_to_receive:
		push_error("No data schema. Node path: ", get_path())
		return;
	
	if get_parent() and not get_parent().is_node_ready():
		await get_parent().ready

	var db = SpacetimeDB.get_local_database()

	if db == null:
		await SpacetimeDB.database_initialized
	else:
		var table_name_str = table_to_receive.get_meta("table_name")
		var data = db.get_all_rows(table_name_str)
		for row_data in data:
			_on_insert(table_name_str, row_data)
			
func _on_insert(_table_name: String, row: ModuleTable):
	if row.get_meta("table_name") != table_to_receive.get_meta("table_name"):
		return
	insert.emit(row)

func _on_update(_table_name: String, row: ModuleTable, previous: ModuleTable):
	if row.get_meta("table_name") != table_to_receive.get_meta("table_name"):
		return
	update.emit(row, previous)

func _on_delete(_table_name: String, row: ModuleTable):
	if row.get_meta("table_name") != table_to_receive.get_meta("table_name"):
		return
	delete.emit(row)
