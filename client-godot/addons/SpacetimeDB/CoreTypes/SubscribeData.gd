extends Resource
class_name SubscribeData

@export var queries: Array[String]

func _init(p_queries: Array[String] = []):
	# Ensure correct type upon initialization if needed
	var typed_queries: Array[String] = []
	typed_queries.assign(p_queries) # Copy elements, ensuring type
	queries = typed_queries
	pass
