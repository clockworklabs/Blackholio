class_name Option extends Resource

@export var data: Array = [] :
	set(value):
		if value is Array:
			if value.size() > 0:
				_internal_data = value.slice(0, 1)
			else:
				_internal_data = []
		else:
			push_error("Optional data must be an Array.")
			_internal_data = []
	get():
		return _internal_data

var _internal_data: Array = []

func is_some() -> bool:
	return _internal_data.size() > 0

func is_none() -> bool:
	return _internal_data.is_empty()

func unwrap():
	if is_some():
		return _internal_data[0]
	else:
		push_error("Attempted to unwrap a None Optional value!")
		return null
		
func unwrap_or(default_value):
	if is_some():
		return _internal_data[0]
	else:
		return default_value
		
func set_some(value):
	self.data = [value]
	
func set_none():
	self.data = []

func to_string() -> String:
	if is_some():
		return "Some(%s [type: %s])" % [_internal_data[0], typeof(_internal_data[0])]
	else:
		return "None"
