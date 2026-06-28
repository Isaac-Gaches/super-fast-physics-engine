# Super-Fast-Physics-Engine
Here is my deterministic physics engine that is cache friendly, multithreaded and using SIMD where possible. Written in Rust and WGSL using my [easy-gpu](https://github.com/Isaac-Gaches/easy-gpu) api. It uses verlet integration for the physics and a uniform grid for the broadphase collision detection. It can be tweaked for higher performace or for higher accuracy.

Below is 80,000 particles at 60fps on my laptop's intel i5-1035G1:
<img width="1020" height="868" alt="Screenshot 2026-06-28 151438" src="https://github.com/user-attachments/assets/ce98be75-a535-445e-b63d-4cfde4ed1d46" />
﻿
