use crate::converters::*;
use crate::diff_line_printer::*;
use crate::diff_maker::*;
use crate::error_handling::*;
use crate::repository::*;
use gtk::ButtonExt as _;
use gtk::CellLayoutExt as _;
use gtk::ContainerExt as _;
use gtk::GtkListStoreExt as _;
use gtk::GtkListStoreExtManual as _;
use gtk::GtkWindowExt as _;
use gtk::PanedExt as _;
use gtk::TextBufferExt as _;
use gtk::TreeModelExt as _;
use gtk::TreeSelectionExt as _;
use gtk::TextViewExt as _;
use gtk::TreeViewColumnExt as _;
use gtk::TreeViewExt as _;
use gtk::WidgetExt as _;
use std::rc::Rc;

enum FileStatusModelColumn
{
    Status,
    Path
}

const EXPAND_IN_LAYOUT : bool = true;
const SPACING : i32 = 8;
const FILE_STATUS_MODEL_COLUMN_INDICES: [u32; 2] = [
    FileStatusModelColumn::Status as u32,
    FileStatusModelColumn::Path as u32];
const FILE_STATUS_COLUMN_TYPE : gtk::Type = gtk::Type::String;
const FILE_PATH_COLUMN_TYPE : gtk::Type = gtk::Type::String;
const EXCLUDE_HIDDEN_CHARACTERS : bool = false;


pub fn buildGui(gtkApplication: &gtk::Application, repository: Rc<Repository>)
{
    let window = makeWindow(gtkApplication);

    let generalPane = gtk::Paned::new(gtk::Orientation::Horizontal);
    let filesPane = gtk::Paned::new(gtk::Orientation::Vertical);
    let diffAndCommitPane = gtk::Paned::new(gtk::Orientation::Vertical);
    generalPane.add1(&filesPane);
    generalPane.add2(&diffAndCommitPane);

    window.add(&generalPane);

    let fileStatusModels = makeFileStatusModels(&repository);

    let unstagedVerticalBox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
    filesPane.add1(&unstagedVerticalBox);
    unstagedVerticalBox.add(&gtk::Label::new("Unstaged:"));
    let unstagedFilesStatusView = makeUnstagedFilesStatusView(fileStatusModels.clone(), repository.clone());
    unstagedVerticalBox.add(&*unstagedFilesStatusView);

    let stagedVerticalBox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
    filesPane.add2(&stagedVerticalBox);
    stagedVerticalBox.add(&gtk::Label::new("Staged:"));
    let stagedFilesStatusView = makeStagedFilesStatusView(fileStatusModels.clone(), repository.clone());
    stagedVerticalBox.add(&*stagedFilesStatusView);

    let diffVerticalBox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
    diffAndCommitPane.add1(&diffVerticalBox);
    diffVerticalBox.add(&gtk::Label::new("Diff:"));
    let diffView = makeDiffView();
    diffView.set_hexpand(true);
    diffVerticalBox.add(&*diffView);

    let commitVerticalBox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
    diffAndCommitPane.add2(&commitVerticalBox);
    commitVerticalBox.add(&gtk::Label::new("Commit message:"));
    let commitMessageView = gtk::TextView::new();
    commitMessageView.set_vexpand(true);
    commitVerticalBox.add(&commitMessageView);
    makeCommitButton(commitMessageView, repository.clone(), &commitVerticalBox, fileStatusModels.staged);

    setupFileViews(unstagedFilesStatusView, &stagedFilesStatusView, diffView, repository);

    window.show_all();
}

fn makeWindow(gtkApp: &gtk::Application) -> gtk::ApplicationWindow
{
    let window = gtk::ApplicationWindow::new(gtkApp);
    window.set_title("Rusty Git Gui");
    window.set_default_size(400, 400);
    window
}

#[derive(Clone)]
struct FileStatusModels
{
    unstaged: Rc<gtk::ListStore>,
    staged: Rc<gtk::ListStore>
}

fn makeFileStatusModels(repository: &Repository) -> FileStatusModels
{
    let fileInfos = repository.collectFileInfos();

    FileStatusModels {
        unstaged: makeFileStatusModel(&fileInfos.unstaged),
        staged: makeFileStatusModel(&fileInfos.staged)
    }
}

fn makeFileStatusModel(fileInfos: &[FileInfo]) -> Rc<gtk::ListStore>
{
    let fileInfosForModel = fileInfos.iter().map(
        |fileInfo| [&fileInfo.status as &gtk::ToValue, &fileInfo.path as &gtk::ToValue]).collect::<Vec<_>>();

    let fileStatusModel = Rc::new(gtk::ListStore::new(&[FILE_STATUS_COLUMN_TYPE, FILE_PATH_COLUMN_TYPE]));
    for fileInfo in fileInfosForModel {
        fileStatusModel.set(&fileStatusModel.append(), &FILE_STATUS_MODEL_COLUMN_INDICES[..], &fileInfo);
    };
    fileStatusModel
}

struct StagingAreaChangeModels
{
    source: Rc<gtk::ListStore>,
    target: Rc<gtk::ListStore>
}

fn makeUnstagedFilesStatusView(fileStatusModels: FileStatusModels, repository: Rc<Repository>) -> Rc<gtk::TreeView>
{
    makeFilesStatusView(
        StagingAreaChangeModels{
            source: fileStatusModels.unstaged,
            target: fileStatusModels.staged},
        move |path| repository.stageFile(path))
}

fn makeStagedFilesStatusView(fileStatusModels: FileStatusModels, repository: Rc<Repository>) -> Rc<gtk::TreeView>
{
    makeFilesStatusView(
        StagingAreaChangeModels{
            source: fileStatusModels.staged,
            target: fileStatusModels.unstaged},
        move |path| repository.unstageFile(path))
}

fn makeFilesStatusView(
    models: StagingAreaChangeModels,
    stagingStateChanger: impl Fn(&str) + 'static) -> Rc<gtk::TreeView>
{
    let fileStatusView = Rc::new(gtk::TreeView::new_with_model(&*models.source));
    fileStatusView.set_vexpand(true);
    appendColumn("Status", &fileStatusView);
    appendColumn("File", &fileStatusView);
    fileStatusView.connect_row_activated(move |_view, row, _column|
        changeStagingState(&models, row, &stagingStateChanger));
    fileStatusView
}

fn appendColumn(title: &str, view: &gtk::TreeView)
{
    let renderer = gtk::CellRendererText::new();
    let column = gtk::TreeViewColumn::new();
    column.pack_start(&renderer, EXPAND_IN_LAYOUT);
    column.set_title(title);
    column.add_attribute(&renderer, "text", toI32(view.get_n_columns()));
    view.append_column(&column);
}

fn makeDiffView() -> Rc<gtk::TextView>
{
    let diffView = Rc::new(gtk::TextView::new());
    diffView.set_editable(false);
    diffView.set_monospace(true);
    diffView.set_vexpand(true);
    diffView
}

fn makeCommitButton(
    commitMessageView: gtk::TextView,
    repository: Rc<Repository>,
    layoutBox: &gtk::Box,
    stagedFilesModel: Rc<gtk::ListStore>)
{
    let commitButton = gtk::Button::new_with_label("Commit");
    commitButton.connect_clicked(move |_button| commitChanges(&commitMessageView, &repository, &stagedFilesModel));
    layoutBox.add(&commitButton);
}

fn setupFileViews(
    unstagedFilesView: Rc<gtk::TreeView>,
    stagedFilesView: &Rc<gtk::TreeView>,
    diffView: Rc<gtk::TextView>,
    repository: Rc<Repository>)
{
    let stagedFilesViewToUnselect = stagedFilesView.clone();
    connectSelectionChanged(
        &unstagedFilesView,
        diffView.clone(),
        UnstagedDiffMaker{repository: repository.clone()},
        stagedFilesViewToUnselect);

    let unstagedFilesViewToUnselect = unstagedFilesView;
    connectSelectionChanged(
        stagedFilesView,
        diffView,
        StagedDiffMaker{repository},
        unstagedFilesViewToUnselect);
}

fn connectSelectionChanged(
    filesView: &gtk::TreeView,
    diffView: Rc<gtk::TextView>,
    diffMaker: impl DiffMaker + 'static,
    filesViewToUnselect: Rc<gtk::TreeView>)
{
    filesView.get_selection().connect_changed(
        move |selection| handleChangedFileViewSelection(selection, &diffView, &diffMaker, &filesViewToUnselect));
}

fn handleChangedFileViewSelection(
    selection: &gtk::TreeSelection,
    diffView: &gtk::TextView,
    diffMaker: &impl DiffMaker,
    fileViewToUnselect: &gtk::TreeView)
{
    let (rows, model) = selection.get_selected_rows();
    if rows.is_empty() {
        let buffer = diffView.get_buffer()
            .unwrap_or_else(|| exit("Failed to get diff view buffer"));
        buffer.delete(&mut buffer.get_start_iter(), &mut buffer.get_end_iter());
        return;
    }
    else if rows.len() > 1 {
        exit(&format!("Expected file view selection to have at most 1 selected row, got {}.", rows.len()));
    }

    fileViewToUnselect.get_selection().unselect_all();
    displayDiff(&model, diffView, &rows[0], diffMaker);
}

fn displayDiff(
    fileStatusModel: &gtk::TreeModel,
    diffView: &gtk::TextView,
    row: &gtk::TreePath,
    diffMaker: &impl DiffMaker)
{
    let filePath = getFilePathFromFileStatusModel(row, fileStatusModel);
    let diffLinePrinter = DiffLinePrinter::new(diffView);
    let diff = diffMaker.makeDiff(&filePath);
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffLinePrinter.printDiff(&line))
        .unwrap_or_else(|e| exit(&format!("Failed to print diff: {}", e)));
}

fn getFilePathFromFileStatusModel(row: &gtk::TreePath, fileStatusModel: &gtk::TreeModel) -> String
{
    let iterator = fileStatusModel.get_iter(row)
        .unwrap_or_else(|| exit(&format!("Failed to get iterator from file status model for row {}", row)));
    fileStatusModel.get_value(&iterator, FileStatusModelColumn::Path as i32).get().
        unwrap_or_else(|| exit(&format!("Failed to get value from file status model for iterator {:?}, column {}",
            iterator, FileStatusModelColumn::Path as i32)))
}

fn changeStagingState(
    models: &StagingAreaChangeModels,
    row: &gtk::TreePath,
    stagingStateChanger: impl Fn(&str))
{
    let iterator = models.source.get_iter(row)
        .unwrap_or_else(|| exit(&format!("Failed to get iterator from file status model for row {}", row)));
    let filePath = models.source.get_value(&iterator, FileStatusModelColumn::Path as i32).get::<String>().
        unwrap_or_else(|| exit(&format!("Failed to get value from file status model for iterator {:?}, column {}",
                                        iterator, FileStatusModelColumn::Path as i32)));
    let fileStatus = models.source.get_value(&iterator, FileStatusModelColumn::Status as i32).get::<String>().
        unwrap_or_else(|| exit(&format!("Failed to get value from file status model for iterator {:?}, column {}",
                                        iterator, FileStatusModelColumn::Status as i32)));

    stagingStateChanger(&filePath);
    models.source.remove(&iterator);
    models.target.set(&models.target.append(), &FILE_STATUS_MODEL_COLUMN_INDICES[..],
                      &[&fileStatus as &gtk::ToValue, &filePath as &gtk::ToValue]);
}

fn commitChanges(commitMessageView: &gtk::TextView, repository: &Repository, stagedFilesModel: &gtk::ListStore)
{
    let buffer = commitMessageView.get_buffer()
        .unwrap_or_else(|| exit("Failed to get commit message view buffer"));

    let message = buffer.get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), EXCLUDE_HIDDEN_CHARACTERS)
        .unwrap_or_else(|| exit(&format!("Failed to get text from commit message view buffer")));
    repository.commitChanges(&message);

    stagedFilesModel.clear();
    buffer.delete(&mut buffer.get_start_iter(), &mut buffer.get_end_iter());
}