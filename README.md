# neat
A basic physics sim that will eventually become a platform for learning about the neat algorithm. i.e. using it to teach a quadcopter how to fly.

## Usage
#### [Documentation](http://trolleyman.github.io/docs/neat/)

`cargo run --release` to run the default simulation

`cargo run --release --example <example>` to run \<example\>. [List of examples](examples) (without the '.rs')

## Arguments
- `-p` pauses the simulation
- `-v` makes it verbose

## Key bindings
- `F1` to resume the simulation
- `F2` to step the simulation
- `F3` to toggle wireframe mode
- `F4` to reload the GLSL shaders
- `F5` to reset the state