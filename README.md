# Vibe Enginereed Rust 3D Wireframe for Wasm

<img width="1468" height="723" alt="Screenshot 2026-09-17 at 19 39 00" src="https://github.com/user-attachments/assets/10934d42-8c38-4480-99ca-3cf6ccd9d31f" />

So this project was meant to understand better how you can build a 3D application in Rust with Wasm as the sole target, while making use of Egui as the GUI motor.

The steps to build this were outlined in the AGENTS.md file. We built it up slowly. I wanted to keep the engine itself as simple as possible, not bothering yet with camera, dynamic FOV, solid objects, rasterization, lighting or even clipping or culling. I didn't even want to use matrix multiplications here just to keep things as simple as I could. I didn't even want to add rotation controls - but I couldn't resist.

It is based on a simple list of vertices and a list of polygons that together make up the entire structure of a 3d object. The torus factory then takes this structure and generates a torus based on the specified parameters. To keep things so simple is a good way to see how it was all built up and how something like this can be done. I tried to git commit every discrete step as well as I could so that the parts of building something like this up can be studied in isolation. I also hosted a version here (I had to tinker with the index.html manually though to get my server to understand the refs):https://zzp-online-marketing.nl/torust/. I couldn't resist the ToRust pun.
