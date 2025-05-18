class_name EntityController extends Node2D

const LERP_DURATION_SEC: float = 0.1

var color: Color
var entity_id: int
var lerp_time: float
var lerp_start_position: Vector3
var lerp_target_position: Vector3
var target_scale: Vector3

func spawn_entity(input_entity_id: int):
	entity_id = input_entity_id
	var db = SpacetimeDB.get_local_database()
	var entity = db.get_row("entity", input_entity_id)
	lerp_start_position = entity.position
	lerp_target_position = entity.position
	global_position = entity.position
	scale = Vector2.ONE
	target_scale = mass_to_scale(entity.mass)

func on_delete(context):
	# TODO: if the context is Reducer.ConsumeEntity, make a shrink out effect
	print(context)
	queue_free()

func _process(delta: float):
	lerp_time = min(lerp_time + delta, LERP_DURATION_SEC)
	position = lerp(lerp_start_position, lerp_target_position, lerp_time / LERP_DURATION_SEC)
	scale = lerp(scale, target_scale, delta * 8)

func mass_to_scale(mass: int):
	var diameter = mass_to_diameter(mass)
	return Vector3(diameter, diameter, 1)

func mass_to_radius(mass: int): sqrt(mass);
func mass_to_diameter(mass: int): mass_to_radius(mass) * 2;
