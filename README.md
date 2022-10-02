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

A `Table` holds both `Col` and `Row` objects. Tables in Stanza are row-oriented; i.e., a `Col` is used purely for assigning styles; `Row` is where the data lives.

A `Row` contains a vector of `Cell`s. In turn, a `Cell` houses a `Content` enum. The simplest and most common content type that you will use is a `Content::Label`.

Let's start by creating a most basic, 2x3 table.

```rust
use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::table::Table;

// build a table model
let table = Table::default()
    .with_row(vec!["Department", "Budget"])
    .with_row(vec!["Sales", "90000"])
    .with_row(vec!["Engineering", "270000"]);

// configure a renderer that will later turn the model into a string
let renderer = Console::default();

// render the table, outputting to stdout
println!("{}", renderer.render(&table, &[]));
```

The resulting output:
```c
╔═══════════╤══════╗
║Department │Budget║
╟───────────┼──────╢
║Sales      │90000 ║
╟───────────┼──────╢
║Engineering│270000║
╚═══════════╧══════╝
```

Not bad for a half-dozen of lines. We used all the concepts above without specifying them explicitly. For example, we didn't refer to `Col`, `Cell` or `Content` types at all. Stanza offers a highly abridged syntax for building tables where the additional flexibility mightn't be needed. Still, it's worth understanding what happens under the hood. The exact same table model can be produced using the _fully explicit syntax_ below.

```rust
use stanza::style::Styles;
use stanza::table::{Cell, Col, Content, Row, Table};

Table::with_styles(Styles::default())
    .with_cols(vec![
        Col::new(Styles::default()),
        Col::new(Styles::default()),
    ])
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::new(
                Styles::default(),
                Content::Label(String::from("Department")),
            ),
            Cell::new(Styles::default(), Content::Label(String::from("Budget"))),
        ],
    ))
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::new(Styles::default(), Content::Label(String::from("Sales"))),
            Cell::new(Styles::default(), Content::Label(String::from("90000"))),
        ],
    ))
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::new(
                Styles::default(),
                Content::Label(String::from("Engineering")),
            ),
            Cell::new(Styles::default(), Content::Label(String::from("270000"))),
        ],
    ));
```

We've gone from 4 lines to over 30, but that's what it takes to specify this model using the fully explicit syntax. You'll be pleased to know that Stanza's syntax is not a binary _either-or_: it lets you progressively use more explicit constructs as you need to refine your styles or layouts. Note also that we didn't need to specify columns in the first cut — the table model will autogenerate the column definitions based on the number of cells.

Our table lacks a few niceties, however. Ideally, we would like to top row to act as a header. The context is also a little cramped. And finally, the budget figures should ideally be right-aligned. Lets a build a more stylish table.

```rust
use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{HAlign, Header, MinWidth, Styles};
use stanza::table::{Cell, Col, Row, Table};

let table = Table::default()
    .with_cols(vec![
        Col::new(Styles::default().with(MinWidth(20))),
        Col::new(Styles::default().with(MinWidth(15)).with(HAlign::Right))
    ])
    .with_row(Row::new(
        Styles::default().with(Header(true)),
        vec![Cell::from("Department"), Cell::from("Budget")],
    ))
    .with_row(vec!["Sales", "90000"])
    .with_row(vec!["Engineering", "270000"]);

let renderer = Console::default();
println!("{}", renderer.render(&table, &[]));
```

```c
╔════════════════════╤═══════════════╗
║Department          │         Budget║
╠════════════════════╪═══════════════╣
║Sales               │          90000║
╟────────────────────┼───────────────╢
║Engineering         │         270000║
╚════════════════════╧═══════════════╝
```

Earlier, it was promised that you could output the same table model into a variety of formats. To switch to Markdown, simply replace the renderer:

```rust
use stanza::renderer::markdown::Markdown;
use stanza::renderer::Renderer;

// build the model ...

let renderer = Markdown::default();

// render ...
```

```c
|Department          |         Budget|
|:-------------------|--------------:|
|Sales               |          90000|
|Engineering         |         270000|
```

Voilà, we have Markdown!

This is a good segue into styles. Stanza is build around the philosophy of separating models from renderers. While this offers phenomenal flexibility, it does create a problem. Every renderer is different

```c
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