use crate::style::Styles;

#[derive(Default)]
pub struct Table {
    styles: Styles,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(styles: Styles, rows: Vec<Row>) -> Self {
        Self { styles, rows }
    }

    pub fn with_styles(styles: Styles) -> Self {
        Self::new(styles, Vec::default())
    }

    pub fn with_row(mut self, row: Row) -> Self {
        self.push_row(row);
        self
    }

    pub fn push_row(&mut self, row: Row) {
        self.rows.push(row)
    }

    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn num_cols(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.cells().map_or(0, |cells| cells.len()))
            .max()
            .unwrap_or(0)
    }

    pub fn cell(&self, col: usize, row: usize) -> Option<&Cell> {
        if row >= self.rows.len() {
            return None;
        }
        let row = &self.rows[row];
        let cells = row.cells();
        match cells {
            None => None,
            Some(cells) => cells.get(col)
        }
    }

    pub fn is_empty(&self) -> bool {
        self.num_rows() == 0 || self.num_cols() == 0
    }
}

pub enum Row {
    Header(Styles, Vec<Cell>),
    Body(Styles, Vec<Cell>),
    Separator(Styles),
}

impl Row {
    pub fn cells(&self) -> Option<&[Cell]> {
        match self {
            Row::Header(_, cells) => Some(cells),
            Row::Body(_, cells) => Some(cells),
            Row::Separator(_) => None,
        }
    }
}

// #[derive(Default)]
// pub struct Cells(Vec<Cell>);
//
// impl From<Vec<Cell>> for Cells {
//     fn from(vec: Vec<Cell>) -> Self {
//         Self(vec)
//     }
// }
//
// impl Cells {
//     pub fn slice(&self) -> &[Cell] {
//         &self.0
//     }
// }

pub struct Cell {
    styles: Styles,
    data: String,
}

impl Cell {
    pub fn new(styles: Styles, data: String) -> Self {
        Self { styles, data }
    }

    pub fn styles(&self) -> &Styles {
        &self.styles
    }

    pub fn data(&self) -> &str {
        &self.data
    }
}

impl<S: ToString> From<S> for Cell {
    fn from(data: S) -> Self {
        Self::new(Styles::default(), data.to_string())
    }
}
