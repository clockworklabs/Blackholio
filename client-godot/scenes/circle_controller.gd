class_name CircleController extends EntityController

@export var color_palette: Array[Color] = [
	# Yellow
	Color(175, 159, 49, 255),
	Color(175, 116, 49, 255),
	# Purple
	Color(112, 47, 252, 255),
	Color(51, 91, 252, 255),
	# Red
	Color(176, 54, 54, 255),
	Color(176, 109, 54, 255),
	Color(141, 43, 99, 255),
	# Blue
	Color(2, 188, 250, 255),
	Color(7, 50, 251, 255),
	Color(2, 28, 146, 255),
]

var player_owner: PlayerController

func spawn(circle: BlackholioCircle, input_owner: PlayerController):
	super.spawn(circle.entity_id)
	color = fmod(circle.player_id, color_palette.size())
	player_owner = input_owner
	# TODO: Set the label for this circle GetComponentInChildren<TMPro.TextMeshProUGUI>().text = owner.Username;

func on_delete():
	owner.on_circle_deleted(self)
	queue_free()
