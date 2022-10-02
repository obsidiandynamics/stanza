# Stanza
An abstract table model written in Rust, with customisable text formatting and renderers.

# Why Stanza
* **Feature-complete**: Stanza supports a broad range of styling features — various text formatting controls, foreground/background/fill colours, border styles, multiple horizontal and vertical headers and separators, and even nested tables, to name a few.
* **Pluggable renderers**: The clean separation of the table model from the render implementation lets you switch between output formats. For example, the table you use to output to the terminal can be switched to produce a Markdown document instead. You can also add your own render; e.g., to output HTML or paint a TUI/Curses screen.
* **Ease of use**:  Simple things are easy to do and hard things are possible. Stanza offers both a fluid API for building "static" tables and an API for building a table programmatically.
* **No standard library needed**: Stanza is `no_std`, meaning it can be used in embedded devices.
* **Performance**: It takes ~10 µs to build the table model used in the screenshot above and ~200 µs to render it. (Markdown takes roughly half that time.) Efficiency mightn't sound like a concern in desktop and server use cases, but it makes a difference in low-powered devices.

# Getting Started
## Add dependency
```sh
cargo add stanza
```

## A basic table
Stanza intentionally separates the `Table` model from the set of optional `Style` modifiers and, finally, the `Renderer` that is used to turn the table into a printable `Display` object.

There are four main types of elements within the model: `Table`, `Col`, `Row` and `Cell`.

A `Row` contains a vector of `Cell`s. In turn, a `Cell` contains a `Content` enum. The simplest and most common content type that you will use is a `Content::Label`.

Let's start by creating a most elemental, 2x3 table.



```
Table
  └─ Col
       └─ Row
            └─ Cell
```
`Table`

* Column
* Row
* Cell

In principle, styles may be applied to any table element. In practice, some styles are

Styles that apply to the table will also apply to all elements contained within it