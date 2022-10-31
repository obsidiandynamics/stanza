use crate::style::{Assignability, Style, Styles};

#[test]
fn assignability() {
    assert!(!Assignability::TableOnly.at_col());
    assert!(!Assignability::TableOnly.at_row());
    assert!(!Assignability::TableOnly.at_cell());

    assert!(Assignability::ColTable.at_col());
    assert!(!Assignability::ColTable.at_row());
    assert!(!Assignability::ColTable.at_cell());

    assert!(!Assignability::RowTable.at_col());
    assert!(Assignability::RowTable.at_row());
    assert!(!Assignability::RowTable.at_cell());

    assert!(Assignability::RowColTable.at_col());
    assert!(Assignability::RowColTable.at_row());
    assert!(!Assignability::RowColTable.at_cell());

    assert!(Assignability::CellRowColTable.at_col());
    assert!(Assignability::CellRowColTable.at_row());
    assert!(Assignability::CellRowColTable.at_cell());
}

#[derive(Debug, Clone)]
struct SampleStyleOne;

#[derive(Debug, Clone)]
struct SampleStyleTwo;

impl Style for SampleStyleOne {
    fn assignability(&self) -> Assignability {
        unimplemented!()
    }
}

impl Style for SampleStyleTwo {
    fn assignability(&self) -> Assignability {
        unimplemented!()
    }
}

#[test]
fn resolve() {
    let styles = Styles::default().with(SampleStyleOne);
    assert!(SampleStyleOne::resolve(&styles).is_some());
    assert!(SampleStyleTwo::resolve(&styles).is_none());
}