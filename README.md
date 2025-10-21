## Overview
A lightweight pixel-based game built with the **winit** and **pixels** crates, mimicking [flappy bird](https://en.wikipedia.org/wiki/Flappy_Bird)

<img width="893" height="595" alt="Screenshot 2025-10-20 at 9 34 05 PM" src="https://github.com/user-attachments/assets/2e41e35b-9600-4121-a593-185aa8e5d7ab" />

Want to play? Pull the code, [install rust](https://rust-lang.org/tools/install/), and run:
```
cargo run
```

I want to learn rust, and this was my first (non-school) project. 

## Some Notes
- I decided to use winit and pixels because I wanted to use something lighter than a full game engine (e.g. bevy). pixels was perfect for me because all I wanted was a pixel buffer to draw to. 

## Things for the future / improve on
- Optimizations. A lot of the code was written to get the job done. Some of it could be rewritten to avoid loops for example, but it is performant and fast, thanks to rust :)
- Better code organization. I think the organization is fine, but I this I could have separated the different game states into their own scenes and render them from a manager struct or something. Something that other game engines do.
- Graphics. It would be nice if it wasn't just squares and boxes.
- Other mechanics: High score, CLP inputs for seed and game speed, etc.
