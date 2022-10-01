use std::ops::Deref;
use crate::style::{Styled, Styles};

#[derive(Default)]
pub struct Table {
    styles: Styles,
    cols: Vec<Col>,
    rows: Vec<Row>,
}

impl Styled for Table {
    fn styles(&self) -> &Styles {
        &self.styles
    }
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
        while self.cols.len() < row.1.len() {
            self.cols.push(Col::new(Styles::default()))
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
            .map(|row| row.1.len())
            .max()
            .unwrap_or(0)
    }

    pub fn col(&self, col: usize) -> Element<Col> {
        let parent_styles = vec![&self.styles];
        let col = self.cols.get(col);
        Element {
            parent_styles,
            element: col,
        }
    }

    pub fn row(&self, row_idx: usize) -> Element<Row> {
        let parent_styles = vec![&self.styles];
        let row = self.rows.get(row_idx);
        Element {
            parent_styles,
            element: row,
        }
    }

    pub fn cell(&self, col_idx: usize, row_idx: usize) -> Element<Cell> {
        let col = self.cols.get(col_idx);
        let row = self.rows.get(row_idx);
        let mut parent_styles = vec![&self.styles];

        if let Some(col) = col {
            parent_styles.push(col.styles());
        }

        let cell = match row {
            None => None,
            Some(row) => {
                parent_styles.push(row.styles());
                row.1.get(col_idx)
            }
        };

        Element {
            parent_styles,
            element: cell
        }
    }

    pub fn is_empty(&self) -> bool {
        self.num_rows() == 0 || self.num_cols() == 0
    }
}

pub struct Col(Styles);

impl Col {
    pub fn new(styles: Styles) -> Self {
        styles.assert_assignability::<Self>(|assignability| assignability.at_col());
        Self(styles)
    }
}

impl Styled for Col {
    fn styles(&self) -> &Styles {
        &self.0
    }
}

pub struct Row(Styles, Vec<Cell>);

impl Row {
    pub fn new(styles: Styles, cells: Vec<Cell>) -> Self {
        styles.assert_assignability::<Self>(|assignability| assignability.at_row());
        Self(styles, cells)
    }

    pub fn cells(&self) -> &[Cell] {
        &self.1
    }
}

impl Styled for Row {
    fn styles(&self) -> &Styles {
        &self.0
    }
}

pub struct Cell {
    styles: Styles,
    data: String,
}

impl Styled for Cell {
    fn styles(&self) -> &Styles {
        &self.styles
    }
}

impl Cell {
    pub fn new<S: ToString>(styles: Styles, data: S) -> Self {
        styles.assert_assignability::<Self>(|assignability| assignability.at_cell());
        Self { styles, data: data.to_string() }
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
    element: Option<&'a T>,
}

impl<'a, T: Styled> Element<'a, T> {
    pub fn parent_styles(&self) -> &[&'a Styles] {
        &self.parent_styles
    }

    pub fn blended_styles(&self) -> Styles {
        let mut styles = Styles::default();
        for &s in &self.parent_styles {
            styles.insert_all(s);
        }
        if let Some(element) = self.element {
            styles.insert_all(element.styles());
        }
        styles
    }
}

impl<'a, T: Styled> Deref for Element<'a, T> {
    type Target = Option<&'a T>;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

#[cfg(test)]
mod tests;