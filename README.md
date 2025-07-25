# firmament-agb
 GBA homebrew game framework built from [agb for Rust](https://github.com/agbrs/agb)

 Intended to be cloned and for people to build projects using it as a template.

 Currently a WIP project that is missing only a few core systems to be usable in projects, check [issues page](https://github.com/sitton76/firmament-agb/issues) for more details

 For building and setup, [check chapter 3 of the agb rust book](https://agbrs.dev/book/setup/getting_started.html)

# Adding Objects
 1. Create a new file in the `src/actors/` folder. (example: `new_actor.rs`)
 2. Create a new struct with a constructor for it. (example: `struct NewActor {}`)
 3. Implement the `GameObj` trait for the new struct, implement any function you need, if the default implementation is fine for your object then don`t implement it.
 4. In `src/actor.rs` file, include the new file to give the program access to it. (example: `#[path = "actors/new_actor.rs"] pub(crate) mod new_actor;`
 5. Create a enum value under `Actor` and insert any properties it might need for when it is initialized(example: `ANewActor(Vector2D<i32>)`
 6. In the `spawn_actor()` function add a match case for your enum and return it in a Box (example: `Actor::ANewActor(pos) => Box::new(new_actor::NewActor::new(pos))`

# Using Objects in Scene layouts.
 1. Open the `src/scene.rs` file.
 2. In the SCENES enum, define a new value for your new scene(example: `NewScene001`)
 3. Under the `get_layout()` function below, make a match case for your SCENE enum.
 4. Initialize your objects using the Actor:: enums and insert the default values for them, then push the result into `new_obj_box` (example: `new_obj_box.push(Actor::ANewActor(vec2(100, 100)));` )

# Adding/Using persistant data.
 1. Open the `src/global_data.rs` file.
 2. Add new value to the `GlobalData` struct.
 3. Add any interfacing function needed for it to the GameData struct(getters/setters and such)
 4. Actors that implement `GameObj` have access to GlobalData via the `update()` and `simple_update()` functions, so get and use data as needed from there.

# Switching scenes.
 So to switch scenes all you need to do is queue it, as a example in the `update()` GameObj function you can just do:
 
 `globals.queue_scene_transition(scene::SCENES::NewScene001);` and it should execute on the next main loop update cycle.
