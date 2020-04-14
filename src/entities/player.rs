//tile_position - map-coordinates
//orientation - facing direction
//rigid_body (velocity?) - phys sim items
//player_input - allows players and ai to both be controlled the same with input tweak
//movement_speed - max velocity

//player_movement_system [player_input, movement_speed, rigid_body]

//player entity
//player_input, movement_speed, rigid_body, tile_position, orientation,--- transform, sprite

//component: collider
//physics_system [collider, tile_position, rigid_body]
//check: out of bounds (zoning)

//take in the player input, convert to a velocity
//check if a collision occurs, if so, set velocity to zero/ else update position
pub struct Player;
// tile_position
// sprite
// collidable
// input
// player_character
// motion
// orientation
