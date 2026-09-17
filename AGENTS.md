# Wasm 3D UI

This project will contain a very simple 3D wireframe renderer, displaying a torus, controlled with some simple UI controls, built in Rust and compiled only for Wasm to serve as a web app.

Here are the steps in which we will set this up:

1. [x] We will scaffold the project. We will use `egui` for the UI layer and `wgpu` for graphics. We will have to set up the project such that we target only compilation for Wasm that can run as a web app, no need to target for native build. Create `scaffold.sh` and `build-and-run.sh` recording the setup/`cargo run` commands, with brief comments on terse flags. Create a `./check.sh` file as well, where we can run fmt and clippy. Remember build and run should also build the Wasm target and serve it.
2. [x] We will add a class that can handle graphics. It should instantiate the screen, rendering and drawing lines. To test it, we should instantiate the screen and draw a vertical line
3. [x] We will have to build support for 3D objects. They should contain a vlist and plist for the vertices and the polygons with referents to the vertex list, building up each polygon based on them. Each object can have several polygons that all point to entries in the same vlist. The object should also have a color, x and y.
4. [x] We will add a simple 3d rendering engine:
   - It should handle only wireframe
   - FOV can be assumed to be 90 degrees to keep it simple.
   - We do not need to implement a camera, assuming the view is at 0,0,0 and the direction is 0,0,-1 into the screen on the z-axis.
   - We do not need to perform matrix calculations just yet, so any translations should be done by hand right now. Let's in fact not consider translations at the moment just to keep things simple.
   - We can assume that no object will ever exceed the view frustrum, so no need to think about clipping or culling of any kind.
   - The engine should clear the screen and render any object each frame.
   - We can translate from object space right down to screen space in one go, keeping it as short as we can.
5. [x] We will build a class that can yield a torus object. We can base it off the `resources/create-rings.js` file. Let's make sure it is indeed configurable in terms of sides, turns, thickness, reach and color. Since this is wireframe, we only need one color.
6. [x] We will no longer show the line on the screen but we will show an instance of the Torus as a 3D object, projected onto the screen.
7. [x] We will add UI controls to our screen with which we can control the sides, turns, thickness, reach and color. These must all be numeric inputs except for the color which can be a string input accepting a hexadecimal value. If we see it is not a proper 6-hex value, we an default the chosen color to white.
8. [x] Each time the user changes one of the UI inputs, we destroy the current Torus 3D object instance we have and we create a new one, based on the updated inputs.
9. [ ] Write a report on the codebase after a thorough investigation with actionable advice.
