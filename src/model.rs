use crate::style::{Styled, Styles};

#[derive(Default)]
pub struct Table {
    styles: Styles,
    cols: Vec<Col>,
    rows: Vec<Row>,
}

impl Table {
    pub fn new(styles: Styles, cols: Vec<Col>, rows: Vec<Row>) -> Self {
        Self { styles, cols, rows }
    }

    pub fn with_styles(styles: Styles) -> Self {
        Self::new(styles, Vec::default(), Vec::default())
    }

    pub fn with_cols(mut self, cols: Vec<Col>) -> Self {
        self.set_cols(cols);
        self
    }

    pub fn with_row(mut self, row: Row) -> Self {
        if let Some(cells) = row.cells() {
            while self.cols.len() < cells.len() {
                self.cols.push(Col::Body(Styles::default()))
            }
        }
        self.push_row(row);
        self
    }

    pub fn set_cols(&mut self, cols: Vec<Col>) {
        let widest_row = self.compute_widest_row();
        assert!(
            cols.len() >= widest_row,
            "Cannot assign fewer than {widest_row} columns"
        );
        self.cols = cols;
    }

    pub fn push_row(&mut self, row: Row) {
        self.rows.push(row)
    }

    pub fn num_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn num_cols(&self) -> usize {
        self.cols.len()
    }

    fn compute_widest_row(&self) -> usize {
        self.rows
            .iter()
            .map(|row| row.cells().map_or(0, |cells| cells.len()))
            .max()
            .unwrap_or(0)
    }

    pub fn col(&self, col: usize) -> Option<Element<Col>> {
        let col = self.cols.get(col);
        match col {
            None => None,
            Some(col) => {
                let parent_styles = vec![&self.styles, col.styles()];
                Some(Element {
                    parent_styles,
                    element: col,
                })
            }
        }
    }

    pub fn cell(&self, col: usize, row: usize) -> Option<Cell> {
        if row >= self.rows.len() {
            return None;
        }
        let row = &self.rows[row];
        let cells = row.cells();
        match cells {
            None => None,
            Some(cells) => {
                let cell = cells.get(col);
                match cell {
                    None => None,
                    Some(cell) => {
                        let mut styles = Styles::default();
                        styles.insert_all(&self.styles);
                        styles.insert_all(self.cols[col].styles());
                        styles.insert_all(row.styles());
                        styles.insert_all(&cell.styles);
                        Some(Cell::new(styles, cell.data.clone()))
                    }
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.num_rows() == 0 || self.num_cols() == 0
    }
}

pub enum Col {
    Header(Styles),
    Body(Styles),
    Separator(Styles),
}

impl Styled for Col {
    fn styles(&self) -> &Styles {
        match self {
            Col::Header(styles) => styles,
            Col::Body(styles) => styles,
            Col::Separator(styles) => styles,
        }
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

    pub fn styles(&self) -> &Styles {
        match self {
            Row::Header(styles, _) => styles,
            Row::Body(styles, _) => styles,
            Row::Separator(styles) => styles,
        }
    }
}

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

pub struct Element<'a, T: Styled> {
    parent_styles: Vec<&'a Styles>,
    element: &'a T,
}

impl<'a, T: Styled> Element<'a, T> {
    pub fn parent_styles(&self) -> &[&'a Styles] {
        &self.parent_styles
    }

    pub fn get(&self) -> &T {
        &self.element
    }

    pub fn combined_styles(&self) -> Styles {
        let mut styles = Styles::default();
        for &s in &self.parent_styles {
            styles.insert_all(s);
        }
        styles.insert_all(self.element.styles());
        styles
    }
}
