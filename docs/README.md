# Exploring diagrams-as-code as a viable alternative to WYSIWYG diagram editors

This directory uses the Python library `diagrams` in a containerized environment, to explore it's viability as a relevant tool for diagramming as documentation as well as to compare to other WYSIWYG diagram editors.

> Note: `diagrams` does not support `.svg` export for certain icons. You can still export in `.svg` but if the icons are `.png` you may be SOL.

## Pros

- Single-origin source
- Built-in icons for popular cloud infra such as AWS, Azure, GCloud, k8s and more

## Cons

- The dependency, `graphviz`, does not have support to export to vector graphics formats such as `.svg` when icons are in `.png` format. [Ref: embedded images are not `.svg`](https://github.com/mingrammer/diagrams/issues/26)
- `draw.io` also supports icons imported through other libraries: [draw.io announcement](https://www.drawio.com/blog/aws-diagrams)

## Purpose

The purpose of this experiment is to see if there is a better developer experience (DX) for using a diagrams-as-code tool vs. using a WYSIWYG editor.

I personally prefer using tools that support `svg` output so we shall see if using `diagrams` is worth it in the long run. I just don't want to waste the containerized build for docs.
