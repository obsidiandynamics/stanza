use stanza::renderer::console::{Console, Decor};
use stanza::renderer::Renderer;
use stanza::table::Table;

fn main() {
    // build a table model
    let table = Table::default()
        .with_row(["Department", "Budget"])
        .with_row(["Sales", "90000"])
        .with_row(["Engineering", "270000"]);

    // configure a renderer that will later turn the model into a string
    let renderer = Console(
        Decor::default()
            .suppress_outer_border()
            .suppress_inner_horizontal_border()
            .suppress_all_lines(),
    );

    // render the table, outputting to stdout
    println!("{}", renderer.render(&table));
}
