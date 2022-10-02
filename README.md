# Stanza
An abstract table model written in Rust, with customisable text formatting and renderers.

# Why Stanza
* **Feature-complete**: Stanza supports a broad range of styling features — various text formatting controls, foreground/background/fill colours, border styles, multiple horizontal and vertical headers and separators, and even nested tables, to name a few.
* **Pluggable renderers**: The clean separation of the table model from the render implementation lets you switch between output formats. For example, the table you use to output to the terminal can be switched to output to a Markdown document instead. You can also add your own render; e.g., to output to HTML or a TUI/Curses app.
* **Ease of use**:  Simple things are easy to do and hard things are possible. Stanza offers both a fluid API for building "static" tables and an API for building a table programmatically.
* **No standard library needed**: Stanza is `no_std`, meaning it can be used in embedded devices.
* **Performance**: It takes ~10 µs to build the table model used in the screenshot above and ~200 µs to render it. Efficiency mightn't sound like a concern in desktop and server use cases, but it makes a difference in low-powered devices.