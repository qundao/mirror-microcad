# Preface

In the 2010s, 3D printing suddenly became accessible to almost everyone —
whether in maker spaces or even at home.
People began building their own DIY printers and writing software to control
them. Naturally, these developers needed a platform to design 3D models that
could be printed.

Around that time, [someone](https://github.com/kintel) came up with an idea: a
programming language made specifically for people who prefer working with a
keyboard rather than a mouse or graphics tablet.
That language is called [OpenSCAD](https://github.com/openscad/openscad).
It turned out to be a great success, inspiring countless impressive projects.

We loved it too and created many “thingies” with it.
However, as experienced software engineers, we also had a few points of
critique.
While OpenSCAD is easy to learn and has a syntax reminiscent of C, we felt the
language could be improved in several ways:

- more specialization for creating graphics,  
- better support for modular programming,  
- strict typing and unit handling,  
- a syntax closer to *Rust* than to *C*,  
- a solid library system,  
- plugin support for other programming languages,  
- and a more powerful visualization concept.

Over the past few years, as we became more familiar with *Rust*, we started
*microcad* as a fun side project.
In 2025, we were fortunate to receive funding to develop a visualization
plugin.

After more than a year of work, we’ve developed and partially implemented
several concepts.
There’s still plenty to do and many more ideas to explore — but we believe
we’ve now reached a point where it’s time to share our work with others.
