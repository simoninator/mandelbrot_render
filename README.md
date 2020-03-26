[![Build Status](https://travis-ci.org/simoninator/mandelbrot_render.svg?branch=master)](https://travis-ci.org/simoninator/mandelbrot_render)

A simple Mandelbrot renderer
----------------------------

It is a small project to get used to the Rust programming language and its ecosystem.

Usage:
```bash
$ mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT
```

Example:
```bash
$ mandelbrot mandel.png 1440x900 -1.20,0.35 -1.0,0.20
```

And it looks like:

![mandelbrot](./mandel.png)

