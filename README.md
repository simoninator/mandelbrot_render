[![Build Status](https://travis-ci.org/simoninator/mandelbrot_render.svg?branch=master)](https://travis-ci.org/simoninator/mandelbrot_render) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/simoninator/mandelbrot_render)

A simple [Mandelbrot](https://en.wikipedia.org/wiki/Mandelbrot_set) renderer
----------------------------

It is a small project to get used to the Rust programming language and its ecosystem.

Usage:
```bash
$ mandelbrot FILE PIXELS UPPERLEFT LOWERRIGHT
```

Example:
```bash
$ mandelbrot mandel.png 3000x2000 -2,1 1,-1
```

And it looks like:

![mandelbrot](./example/mandelbrot.png)

