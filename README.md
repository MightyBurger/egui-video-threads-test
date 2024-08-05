# egui-video-threads-test

This repo is a tiny (< 70 lines of code) demonstration of displaying a constantly-updating image using the Rust crate `egui`. One thread updates the image, and another displays it. The crate uses `triple_buffer` to communicate between the threads. The image is just stored as an array of bytes. 

I wrote this to explore testing image sensor hardware using Rust. With "hardware test software", sometimes you get a big 2D array of values and you just want to put some pixels on the screen, darnit.

I'm sharing this repo because I had difficulty figuring out how to get an array of values displayed as pixels on the screen using `egui`. I didn't find many good examples online. Hopefully this saves you a few minutes of searching. Good luck with whatever you're up to.
