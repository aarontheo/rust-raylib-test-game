## Overview

**Project Title**:
raylib-test-game
(bouncing planet gravity simulator?)

**Project Description**:
This project is a simple physics simulation of balls bouncing around a window and influencing each other's motion through gravitational attraction. The project is built using the Rust bindings of the raylib C library, a simple, do-it-yourself game library.

**Project Goals**:
- Write a simulation/game in Rust
- Learn how to use a game framework
- Apply things learned from Rustlings and other Rust tutorials

## Instructions for Build and Use

Steps to build and/or run the software:

1. Ensure you have the latest version of Rust installed.
2. cd into the project directory
3. run the command "cargo run" to build and run the project.

Instructions for using the software:

1. Click anywhere in the window to spawn a ball with a random initial mass, color, and velocity.
2. Notice that the balls will bounce off the walls of the window and influence each other's motion through gravitational attraction!
3. It's pretty, isn't it?

## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Rust compiler version 1.78.0
* Raylib 3.7.0

## Useful Websites to Learn More

I found these websites useful in developing this software:

* [ChatGPT](chat.openai.com)
* [The Rust Book](https://doc.rust-lang.org/book/)
* [Raylib Rust Docs](https://docs.rs/raylib/latest/raylib/)

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] Add a way to remove balls from the simulation
* [ ] Add a preview of the next ball to be spawned
* [ ] Add a way to control the size of the balls in the simulation with the mousewheel
* [ ] Add a way to control the initial velocity of a spawned ball with the mouse
* [ ] Add collision between balls
* [ ] Add a quadtree based collision checker
* [ ] Maybe make gravitational attraction more efficient by implementing an alternative method, such as having a 'gravity field' object with a certain resolution that each ball can affect and sample from to determine the force acting on it
* [ ] If I'm using quadtrees, I may look into implementing a Barnes-Hut algorithm for gravity.
