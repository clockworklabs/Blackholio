class_name FoodController extends EntityController

@export var color_palette: Array[Color] = [
	Color(252, 173, 255),
	Color(250, 146, 255),
	Color(246, 120, 255),

	Color(251, 201, 255),
	Color(249, 184, 255),
	Color(245, 165, 255),
]

func _draw():
	# if !player_owner: return
	draw_circle(Vector2.ZERO, actual_scale.x, color)
	
func spawn(food: BlackholioFood):
	super.spawn_entity(food.entity_id)
	color = color_palette[fmod(food.entity_id, color_palette.size())]
