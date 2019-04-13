use crate::diff_line_printer::DiffLinePrinter;
use crate::diff_maker::DiffMaker;
use crate::error_handling::exit;
use crate::gui_definitions::{FileStatusModelColumn, FILE_STATUS_MODEL_COLUMN_INDICES, StagingAreaChangeModels};
use crate::gui_utils::{clearBuffer,getBuffer};
use crate::repository::Repository;
use failchain::{bail, ResultExt as _};
use failure::ResultExt as _;
use gtk::GtkListStoreExt as _;
use gtk::GtkListStoreExtManual as _;
use gtk::TextBufferExt as _;
use gtk::TreeModelExt as _;
use gtk::TreeSelectionExt as _;
use gtk::TreeViewExt as _;


pub type Error = failchain::BoxedError<ErrorKind>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum ErrorKind
{
    #[fail(display = "Failed to clear diff view.")]
    ClearDiffView,
    #[fail(display = "Failed to commit staged changes.")]
    CommitStagedChanges,
    #[fail(display = "Failed to get commit message view buffer.")]
    CommitMessageViewBuffer,
    #[fail(display = "Failed to display diff.")]
    DisplayDiff,
    #[fail(display = "Failed to handle changed file view selection.")]
    HandleChangedFileViewSelection,
    #[fail(display = "Expected file view selection to have at most 1 selected row, got {}.", 0)]
    TooLargeFileViewSelection(usize)
}

impl failchain::ChainErrorKind for ErrorKind
{
    type Error = Error;
}


const EXCLUDE_HIDDEN_CHARACTERS : bool = false;


pub fn handleChangedFileViewSelection(
    selection: &gtk::TreeSelection,
    diffView: &gtk::TextView,
    diffMaker: &impl DiffMaker,
    fileViewToUnselect: &gtk::TreeView)
    -> Result<()>
{
    (||{
        let (rows, model) = selection.get_selected_rows();
        if rows.is_empty() {
            return clearDiffView(&diffView);
        }
        else if rows.len() > 1 {
            bail!(ErrorKind::TooLargeFileViewSelection(rows.len()));
        }

        fileViewToUnselect.get_selection().unselect_all();
        displayDiff(&model, diffView, &rows[0], diffMaker)?;
        Ok(())
    })().chain_err(|| ErrorKind::HandleChangedFileViewSelection)
}

fn clearDiffView(diffView: &gtk::TextView) -> Result<()>
{
    let buffer = getBuffer(diffView).context(ErrorKind::ClearDiffView)?;
    clearBuffer(&buffer);
    Ok(())
}

fn displayDiff(
    fileStatusModel: &gtk::TreeModel,
    diffView: &gtk::TextView,
    row: &gtk::TreePath,
    diffMaker: &impl DiffMaker) -> Result<()>
{
    let filePath = getFilePathFromFileStatusModel(row, fileStatusModel);
    let diffLinePrinter = DiffLinePrinter::new(diffView).context(ErrorKind::DisplayDiff)?;
    let diff = diffMaker.makeDiff(&filePath);
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffLinePrinter.printDiff(&line))
        .unwrap_or_else(|e| exit(&format!("Failed to print diff: {}", e)));
    Ok(())
}

fn getFilePathFromFileStatusModel(row: &gtk::TreePath, fileStatusModel: &gtk::TreeModel) -> String
{
    let iterator = fileStatusModel.get_iter(row)
        .unwrap_or_else(|| exit(&format!("Failed to get iterator from file status model for row {}", row)));
    fileStatusModel.get_value(&iterator, FileStatusModelColumn::Path as i32).get().
        unwrap_or_else(|| exit(&format!("Failed to get value from file status model for iterator {:?}, column {}",
                                        iterator, FileStatusModelColumn::Path as i32)))
}

pub fn changeStagingState(
    models: &StagingAreaChangeModels,
    row: &gtk::TreePath,
    switchStagingOfFileInRepository: impl Fn(&str),
    convertFileStatusAfterStagingSwitch: impl Fn(&str) -> String)
{
    let iterator = models.source.get_iter(row)
        .unwrap_or_else(|| exit(&format!("Failed to get iterator from file status model for row {}", row)));
    let filePath = models.source.get_value(&iterator, FileStatusModelColumn::Path as i32).get::<String>().
        unwrap_or_else(|| exit(&format!("Failed to get value from file status model for iterator {:?}, column {}",
                                        iterator, FileStatusModelColumn::Path as i32)));
    let fileStatus = models.source.get_value(&iterator, FileStatusModelColumn::Status as i32).get::<String>().
        unwrap_or_else(|| exit(&format!("Failed to get value from file status model for iterator {:?}, column {}",
                                        iterator, FileStatusModelColumn::Status as i32)));

    switchStagingOfFileInRepository(&filePath);

    let fileStatus = convertFileStatusAfterStagingSwitch(&fileStatus);
    models.source.remove(&iterator);
    models.target.set(&models.target.append(), &FILE_STATUS_MODEL_COLUMN_INDICES[..],
                      &[&fileStatus as &gtk::ToValue, &filePath as &gtk::ToValue]);
}

pub fn convertFileStatusToStaged(fileStatus: &str) -> String
{
    fileStatus.replace("WT", "INDEX")
}

pub fn convertFileStatusToUnstaged(fileStatus: &str) -> String
{
    fileStatus.replace("INDEX", "WT")
}

pub fn commitStagedChanges(
    commitMessageView: &gtk::TextView,
    repository: &Repository,
    stagedFilesModel: &gtk::ListStore)
    -> Result<()>
{
    (|| -> Result<()> {
        let buffer = getBuffer(commitMessageView).context(ErrorKind::CommitMessageViewBuffer)?;

        let message = buffer.get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), EXCLUDE_HIDDEN_CHARACTERS)
            .unwrap_or_else(|| exit("Failed to get text from commit message view buffer"));
        repository.commitChanges(&message);

        stagedFilesModel.clear();
        clearBuffer(&buffer);
        Ok(())
    })().chain_err(|| ErrorKind::CommitStagedChanges)
}