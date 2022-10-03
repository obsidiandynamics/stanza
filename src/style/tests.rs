use crate::style::Assignability;

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