# Stanza
An abstract table model written in Rust, with customisable text formatting and renderers.

# Why Stanza
* **Feature-complete**: Stanza supports a broad range of styling features — various text formatting controls, foreground/background/fill colours, border styles, multiple horizontal and vertical headers and separators, and even nested tables, to name a few.
* **Pluggable renderers**: The clean separation of the table model from the render implementation lets you switch between output formats. For example, the table that you might output to the terminal can be switched to produce a Markdown document instead. You can also add your own render; e.g., to output HTML or paint a TUI/Curses screen.
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
```html
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

```html
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

```html
|Department          |         Budget|
|:-------------------|--------------:|
|Sales               |          90000|
|Engineering         |         270000|
```

Voilà, we have Markdown!

## Styling
This is a good segue into styles. Stanza is build around the philosophy of separating models from renderers. While this offers phenomenal flexibility, it does create a problem. Every renderer is different, which limits the effective use of styles. For example, the `Console` renderer supports a lot richer output than, say, `Markdown`.

When forming the table model, we almost always have a particular output format in mind. Call it the "preferred" format. It's normal to decorate the table for the preferred format. What we'd also like is to render the table in some alternate format without breaking the output or, worse, causing a `panic` because the alternate render mightn't support some style modifier.

Styles are always optional; it is up to the renderer to make use of a style if it can do so meaningfully. Renderers ignore styles they don't understand and may downgrade a style that isn't fully supported, while preserving the legibility of the content. For example, `Markdown` is oblivious to the `Blink` style, but it will still render the text — albeit without the blinking effect. You might even create your own renderer someday and a bunch of styles that only that renderer understands. This won't affect the existing renderers in the slightest.

To organise styles, Stanza posits two basic rules: **specificity** and **assignability**.

### Specificity
Styles cascade, much like their CSS counterparts. A style assigned at some higher-level element will automatically be applied to all elements in the layers below it. The _specificity_ hierarchy is shown below.

```html
Table
  └─> Col
       └─> Row
            └─> Cell
```

Generally, one element is considered to be higher than another if the former intersects with more elements than the latter. As an example, the table intersects all other elements, hence it is at the top of the specificity order. Conversely, a cell only intersects itself, one row, one column and one table, placing it firmly at the bottom.

Rows and columns aren't so cut and dried, because a table could be very tall or very wide. However, tall tables are far more frequent. In fact, that's how one generally scales a table — by adding rows, not columns. A column will intersect more elements in more cases; therefore, it is higher in the specificity hierarchy.

The specificity hierarchy is used to resolve conflicting styles. Say, we applied a `TextFg::Magenta` style at the table level and `TextFg::Cyan` to the cell. Which style should the cell render with? `Cyan`, of course. This is how a regular spreadsheet would behave. Although the cell inherits styles from the table, the column and the row, its own style overrides any of its parents'.

A style defined at the table level will apply to the table and everything contained within, and may be overridden by any lower-level style. A style defined at the column level will apply to the column and all cells intersected by the column, and may be overridden by a cell style. Similarly, a style defined at the row level will apply to the row and all of its cells, and it may be overridden by the cells equivalently. But what happens when a cell inherit the same type of style from both the column and the row, but does not have an overriding style of its own? The row style takes precedence, as it is more specific.

### Assignability
Although styles cascade in a top-down manner, it doesn't mean that a style may be applied on any element. Take the `Header` style, for example. It may be applied to a row or to a column. (Stanza supports vertical headers.) Might a `Header` be assigned to the table as whole? Yes, in which case all rows and columns would be treated as headers. But a header cell makes no sense. Whether a style may be applied to a particular element can be determined by invoking the `S::assignability()` static trait method for some `S: Style`, returning a variant of the `Assignability` enum. The assignability hierarchy is shown below.

```html
Cell
 ├───> Col
 │       └─┐
 └─> Row   │
      └────┴─> Table
```

A style that can be assigned to a cell can also be assigned to a row, a column and a table. Notable examples include text formatting styles — `Bold`, `Italic`, `Underline`, `Blink`, `Strikethrough`, `HAlign`, as well as some colouring styles — `TextFg`, `TextBg`, `FillBg`.

Above the cell, the hierarchy is disjoint at the column and row elements. Any style that can be assigned to a row can also be applied to a table. Similarly, any style that can be applied to a column can also be assigned to a table. These include `Header` and `Separator`.

A style that can be assigned to a row is not necessarily assignable to a column, and vice versa. For example, `MinWidth` and `MaxWidth` may only be assigned to a column — they make no sense at the row level. (And certainly not at the cell level.)

Finally, there are styles that may only be assigned to a table. These include `BorderFg` and `BorderBg` — used to alter the colour of all borders in the table.

The assignability rule is enforced at runtime. Attempting to assign a nonassignable style will fail with a `panic`. As such, changing the returned `Assignability` value of a style to a more restrictive variant would constitute a breaking change.

With all this in mind, let's create another table that demonstrates overriding styles.

## Text handling
We didn't have to work much to get our text laid out well. Often, all that's needed is a `MinWidth` column style and possibly a `HAlign`. Sometimes we might have lots of text, which pushes our column size out:

```rust
use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{Header, MinWidth, Styles};
use stanza::table::{Cell, Col, Row, Table};

let table = Table::default()
    .with_cols(vec![
        Col::new(Styles::default().with(MinWidth(15))),
        Col::new(Styles::default().with(MinWidth(30))),
    ])
    .with_row(Row::new(
        Styles::default().with(Header(true)),
        vec![Cell::from("Poem"), Cell::from("Extract")],
    ))
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::from("Antigonish"), 
            Cell::from("Yesterday, upon the stair, I met a man who wasn't there! He wasn't there again today, Oh how I wish he'd go away!")
        ]
    ))
    .with_row(Row::new(
        Styles::default(),
        vec![
            Cell::from("The Raven"), 
            Cell::from("Ah, distinctly I remember it was in the bleak December; And each separate dying ember wrought its ghost upon the floor.")
        ]
    ));

println!("{}", Console::default().render(&table, &[]));
```

```html
╔═══════════════╤═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╗
║Poem           │Extract                                                                                                                ║
╠═══════════════╪═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╣
║Antigonish     │Yesterday, upon the stair, I met a man who wasn't there! He wasn't there again today, Oh how I wish he'd go away!      ║
╟───────────────┼───────────────────────────────────────────────────────────────────────────────────────────────────────────────────────╢
║The Raven      │Ah, distinctly I remember it was in the bleak December; And each separate dying ember wrought its ghost upon the floor.║
╚═══════════════╧═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════════╝
```

As we expected, the "Extract" column is too wide. A simple way of dealing with this is to set `MaxWidth` style on the column. Simply append `.with(MaxWidth(40))` to the existing styles:

```html
╔═══════════════╤════════════════════════════════════════╗
║Poem           │Extract                                 ║
╠═══════════════╪════════════════════════════════════════╣
║Antigonish     │Yesterday, upon the stair, I met a man  ║
║               │who wasn't there! He wasn't there again ║
║               │today, Oh how I wish he'd go away!      ║
╟───────────────┼────────────────────────────────────────╢
║The Raven      │Ah, distinctly I remember it was in the ║
║               │bleak December; And each separate dying ║
║               │ember wrought its ghost upon the floor. ║
╚═══════════════╧════════════════════════════════════════╝
```

That's better! But since we're dealing with a poem, we should probably respect the author's choice of line breaks. Stanza supports newline characters, leading to line breaks exactly where you need them. Best of all, you can combine newline characters with `MaxWidth`, resulting in something like this:

```html
╔═══════════════╤════════════════════════════════════════╗
║Poem           │Extract                                 ║
╠═══════════════╪════════════════════════════════════════╣
║Antigonish     │Yesterday, upon the stair,              ║
║               │I met a man who wasn't there!           ║
║               │He wasn't there again today,            ║
║               │Oh how I wish he'd go away!             ║
╟───────────────┼────────────────────────────────────────╢
║The Raven      │Ah, distinctly I remember it was in the ║
║               │bleak December;                         ║
║               │And each separate dying ember wrought   ║
║               │its ghost upon the floor.               ║
╚═══════════════╧════════════════════════════════════════╝
```
