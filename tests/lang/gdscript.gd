# 10 lines 5 code 3 comments 2 blanks
extends CharacterBody2D

# Player speed constant
const SPEED = 300.0

# Called every frame
func _physics_process(delta):
    var velocity = Vector2.ZERO
    move_and_slide()
