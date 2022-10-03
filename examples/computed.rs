use stanza::renderer::console::Console;
use stanza::renderer::Renderer;
use stanza::style::{Header, Styles};
use stanza::table::{Content, Row, Table};
use std::thread;
use std::time::Duration;

fn main() {
    fn current_time() -> String {
        chrono::Utc::now().format("%H:%M:%S").to_string()
    }

    // build the table model
    let table = Table::default()
        .with_row(Row::new(
            Styles::default().with(Header(true)),
            vec!["Early-bound".into(), "Late-bound".into()],
        ))
        .with_row(Row::new(
            Styles::default(),
            vec![
                current_time().into(),
                Content::Computed(Box::new(current_time)).into(),
            ],
        ));

    // wait a little
    thread::sleep(Duration::from_secs(2));

    // render the table
    println!("{}", Console::default().render(&table));
}
