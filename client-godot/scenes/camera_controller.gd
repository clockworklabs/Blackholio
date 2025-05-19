extends Camera2D

@export var menu_camera: Camera2D
@export var menu: Control
@export var target_zoom: Vector2 = Vector2(10.0, 10.0)
@export var target_position: Vector2 = Vector2.ZERO

var world_size: int = 1000:
	set(new_value):
		world_size = new_value
		arena_center_transform = Vector2(world_size / 2, world_size / 2)
		menu_camera.position = arena_center_transform
		menu.position = arena_center_transform
var arena_center_transform := Vector2(world_size / 2, world_size / 2)

func _process(delta: float):
	zoom = lerp(zoom, target_zoom, delta * 2)
	offset = lerp(offset, target_position, delta * 2)

	var local_player = GameManager.local_player
	if (local_player == null || !GameManager.is_connected):
		# Set the camera to be in middle of the arena if we are not connected or 
		# there is no local player
		target_zoom = Vector2(1.0, 1.0)
		offset = arena_center_transform
		target_position = offset
		return
		
	ImGui.Begin("Camera")
	ImGui.Text("Zoom: %s" % target_zoom)
	ImGui.Text("Mass: %s" % GameManager.local_player.total_mass())
	ImGui.Text("Circles: %s" % GameManager.local_player.number_of_owned_circles)
	ImGui.End()

	var center_of_mass = local_player.center_of_mass()
	if (center_of_mass):
		# Set the camera to be the center of mass of the local player
		# if the local player has one
		target_position = Vector2(center_of_mass.x, center_of_mass.y)
		target_zoom = Vector2.ONE * calculate_camera_size(local_player)
	else:
		target_position = arena_center_transform

func calculate_camera_size(local_player: PlayerController):
	var distance_per_mass: float = 0.9
	# TODO: This calculation is wrong
	return distance_per_mass + min(distance_per_mass, local_player.total_mass() / 5) + min(local_player.number_of_owned_circles - distance_per_mass, distance_per_mass) * 30
