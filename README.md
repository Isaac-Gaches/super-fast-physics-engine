# Super-Fast-Physics-Engine

• Written in Rust and WGSL with my [easy-gpu](https://github.com/Isaac-Gaches/easy-gpu) crate.

• Uniform grid for broadphase collision detection to remove O(n^2).

• Flattened and compressed grid for better cahce locallity.

• Narrowphase executed in alternating chunks on seperate threads to parallelise without race conditions.

• Integration and part of grid building performed using AVX2 SIMD.

• Batched renderering.

• Tuned to balance staboloty, performance and accuracy.

Below is 80,000 particles at 60fps on my laptop's intel i5-1035G1:
<img width="1020" height="868" alt="Screenshot 2026-06-28 151438" src="https://github.com/user-attachments/assets/ce98be75-a535-445e-b63d-4cfde4ed1d46" />
﻿
