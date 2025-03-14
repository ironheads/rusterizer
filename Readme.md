# Render
This project implements a basic OpenGL rendering pipeline. 
## Results

### Rasterization

you can use the `Rasterization Demo` following
```bash
> cd dist
> python -m http.server ${PORT}
```

### RayTracing

you can see the result of raytracing in `./image.png`
or following
```bash
> cargo run --release --features=raytracing
```

## dependencies
- rust toolchains
  - you can install the dependencies following [rustup](https://rustup.rs/)
  - if you are in china, and you can not download the crates via `cargo`, maybe you need to follow [change crates.io sources](https://mirrors.tuna.tsinghua.edu.cn/help/crates.io-index.git/)
- python 3
  - http.server (python3 built-in)

## Build

### Rasterization
#### Prerequisites
> you can use the `cargo update` to add the dependencies 
 - trunk
 - python3
 - rustup
 - cargo


```bash
> rustup update
> rustup default nightly
> rustup target add wasm32-unknown-unknown
> ./build.sh
```

### Ray Tracing

```bash
> rustup update
> rustup default nightly
> cargo run --release --features=raytracing
```
### Kudos
Rasterization Part was implemented by following the [ssloy/tinyrenderer](https://github.com/ssloy/tinyrenderer) lessons.

RayTracing Part was implemented by following the [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) lessons.

### Todos
- [ ] make mesh hittable
- [ ] when Rasterization, change objects's position 
- [ ] To be contined.
