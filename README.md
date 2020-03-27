[![Build Status](https://travis-ci.org/simoninator/mandelbrot_render.svg?branch=master)](https://travis-ci.org/simoninator/mandelbrot_render) [![Build status](https://ci.appveyor.com/api/projects/status/c19t456e4oes4l1u?svg=true)](https://ci.appveyor.com/project/simoninator/mandelbrot-render) ![Rust](https://github.com/simoninator/mandelbrot_render/workflows/Rust/badge.svg) ![GitHub code size in bytes](https://img.shields.io/github/languages/code-size/simoninator/mandelbrot_render) [![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com) ![Maintenance](https://img.shields.io/maintenance/yes/2020) ![GitHub last commit](https://img.shields.io/github/last-commit/simoninator/mandelbrot_render)

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

