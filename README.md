# light-rehnda

Rust implementation of a CPU based ray tracer based off ray tracing in a weekend.

Implemented features from both [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) and [Ray Tracing The Next Week](https://raytracing.github.io/books/RayTracingTheNextWeek.html).

### Supportes features include

#### Materials

- Lambertian (diffuse) materials
- Metallic materials (reflective with optional blur/fuzz of the reflection)
- Dielectrics (transparent materials suh as water or glass)
- Constant medium volume (e.g. smoke)
- Emissive materials (area lights)

#### Performance Optimisations

- Multithreaded implementation, scalable to _n_ cores
- Bounding Volume Hierarchy (BVH) acceleration structure to make querying large scenes `O(log n)` rather than `O(n)`

#### Camera Features

- Depth of field
- Motion blur

#### Configuration

- RON (Rusty Object Notation) based configuration for the rendering

## Sample Renders

#### Feature demo of dielectrics, metals, lights and diffuse

![Feature Demo](https://github.com/SJBarrett/light-rehnda/blob/master/renders/feature_demo.jpeg)

#### Cornell Box render (demonstrates Global Illumination)

![](https://github.com/SJBarrett/light-rehnda/blob/master/renders/cornell_box_4096s_600p.jpg)

#### Smoke Demo

![Smoke Demo](https://github.com/SJBarrett/light-rehnda/blob/master/renders/smoke.jpg)

### Random Spheres (BVH acceleration structure test scene)

![Random spheres](https://github.com/SJBarrett/light-rehnda/blob/master/renders/random_spheres.jpg)
