use crate::commit_log_column::CommitLogColumn;
use crate::original_row::OriginalRow;
use crate::selections_comparer::SelectionsComparer;

use gtk::traits::TreeModelExt;
use gtk::traits::TreeSelectionExt;


pub(crate) struct CommitLogSelectionsComparer
{
    first: Option<Option<OriginalRow>>,
    second: Option<Option<OriginalRow>>
}

impl CommitLogSelectionsComparer
{
    pub fn new() -> Self
    {
        Self{first: None, second: None}
    }
}

impl SelectionsComparer for CommitLogSelectionsComparer
{
    fn setFirst(&mut self, selection: &gtk::TreeSelection)
    {
        self.first = Some(getSelectedOriginalRow(selection));
    }

    fn setSecond(&mut self, selection: &gtk::TreeSelection)
    {
        self.second = Some(getSelectedOriginalRow(selection));
    }

    fn areDifferent(&self) -> bool
    {
        match self.first {
            Some(firstRowOpt) => {
                match self.second {
                    Some(secondRowOpt) => {
                        firstRowOpt != secondRowOpt
                    },
                    None => panic!("Cannot compare selections, because the second was not set.")
                }
            },
            None => panic!("Cannot compare selections, because the first was not set.")
        }
    }
}

fn getSelectedOriginalRow(selection: &gtk::TreeSelection) -> Option<OriginalRow>
{
    match selection.selected() {
        Some((model, iter)) => {
            let row = model.value(&iter, CommitLogColumn::OriginalRow.into()).get::<OriginalRow>().unwrap();
            Some(row)
        },
        None => None
    }
}
