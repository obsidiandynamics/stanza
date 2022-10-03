use stanza::renderer::console::{Console, Decor};
use stanza::renderer::{Renderer, RenderHint};
use stanza::style::{HAlign, MinWidth, Separator, Styles};
use stanza::table::{Col, Row, Table};

fn main() {
    let inner_renderer = Console({
        let mut decor = Decor::default();
        decor.draw_outer_border = false;
        decor
    });

    let sensors =  Table::default()
        .with_row(Row::from(["Water", "19.3"]))
        .with_row(Row::from(["Oil", "65.1"]));

    let stocks =  Table::default()
        .with_row(Row::from(["AAPL", "138.20"]))
        .with_row(Row::from(["AMZN", "113.20"]))
        .with_row(Row::from(["IBM", "118.81"]));

    let outer = Table::with_styles(Styles::default().with(HAlign::Centred))
        .with_cols(vec![
            Col::default(),
            Col::new(Styles::default().with(Separator(true)).with(MinWidth(5))),
            Col::default(),
        ])
        .with_row(Row::from(["Sensors", "", "Stocks"]))
        .with_row(Row::new(
            Styles::default(),
            vec![
                inner_renderer.render_with_hints(&sensors, &[RenderHint::Nested]).into(),
                "".into(),
                inner_renderer.render_with_hints(&stocks, &[RenderHint::Nested]).into()
            ],
        ));

    println!("{}", Console::default().render(&outer));
}
