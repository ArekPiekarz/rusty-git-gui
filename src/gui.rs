use crate::converters::*;
use crate::diff_line_printer::*;
use crate::diff_maker::*;
use crate::error_handling::*;
use crate::repository::*;
use gtk::CellLayoutExt as _;
use gtk::ContainerExt as _;
use gtk::GtkListStoreExt as _;
use gtk::GtkListStoreExtManual as _;
use gtk::GtkWindowExt as _;
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

pub fn buildGui(gtkApplication: &gtk::Application, repository: Rc<Repository>)
{
    let window = makeWindow(gtkApplication);

    let verticalBox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
    window.add(&verticalBox);

    let fileStatusModels = makeFileStatusModels(&repository);

    let unstagedLabel = gtk::Label::new("Unstaged:");
    verticalBox.add(&unstagedLabel);
    let unstagedFilesStatusView = makeFilesStatusView(&fileStatusModels.unstaged);
    verticalBox.add(&*unstagedFilesStatusView);

    let stagedLabel = gtk::Label::new("Staged:");
    verticalBox.add(&stagedLabel);
    let stagedFilesStatusView = makeFilesStatusView(&fileStatusModels.staged);
    verticalBox.add(&*stagedFilesStatusView);

    let diffView = makeDiffView();
    verticalBox.add(&*diffView);

    setupFileViews(unstagedFilesStatusView, stagedFilesStatusView, diffView, repository);

    window.show_all();
}

fn makeWindow(gtkApp: &gtk::Application) -> gtk::ApplicationWindow
{
    let window = gtk::ApplicationWindow::new(gtkApp);
    window.set_title("Rusty Git Gui");
    window.set_default_size(400, 400);
    window
}

struct FileStatusModels
{
    unstaged: gtk::ListStore,
    staged: gtk::ListStore
}

fn makeFileStatusModels(repository: &Repository) -> FileStatusModels
{
    let fileInfos = repository.collectFileInfos();

    FileStatusModels {
        unstaged: makeFileStatusModel(&fileInfos.unstaged),
        staged: makeFileStatusModel(&fileInfos.staged)
    }
}

fn makeFileStatusModel(fileInfos: &[FileInfo]) -> gtk::ListStore
{
    let fileInfosForModel = fileInfos.iter().map(
        |fileInfo| [&fileInfo.status as &gtk::ToValue, &fileInfo.path as &gtk::ToValue]).collect::<Vec<_>>();

    let fileStatusModel = gtk::ListStore::new(&[gtk::Type::String, gtk::Type::String]);
    for fileInfo in fileInfosForModel {
        fileStatusModel.set(&fileStatusModel.append(), &FILE_STATUS_MODEL_COLUMN_INDICES[..], &fileInfo);
    };
    fileStatusModel
}

fn makeFilesStatusView(fileStatusModel: &gtk::ListStore) -> Rc<gtk::TreeView>
{
    let fileStatusView = Rc::new(gtk::TreeView::new_with_model(fileStatusModel));
    fileStatusView.set_vexpand(true);
    appendColumn("Status", &fileStatusView);
    appendColumn("File", &fileStatusView);
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
    diffView
}

fn setupFileViews(
    unstagedFilesView: Rc<gtk::TreeView>,
    stagedFilesView: Rc<gtk::TreeView>,
    diffView: Rc<gtk::TextView>,
    repository: Rc<Repository>)
{
    let stagedFilesViewToUnselect = Rc::clone(&stagedFilesView);
    connectSelectionChanged(
        &unstagedFilesView,
        Rc::clone(&diffView),
        UnstagedDiffMaker{repository: Rc::clone(&repository)},
        stagedFilesViewToUnselect);

    let unstagedFilesViewToUnselect = unstagedFilesView;
    connectSelectionChanged(
        &stagedFilesView,
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
    let filePath = getFilePathFromFileStatusView(&row, &fileStatusModel);
    let diffLinePrinter = DiffLinePrinter::new(&diffView);
    let diff = diffMaker.makeDiff(&filePath);
    diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| diffLinePrinter.printDiff(&line))
        .unwrap_or_else(|e| exit(&format!("Failed to print diff: {}", e)));
}

fn getFilePathFromFileStatusView(row: &gtk::TreePath, fileStatusModel: &gtk::TreeModel) -> String
{
    let iterator = &fileStatusModel.get_iter(row)
        .unwrap_or_else(|| exit(&format!("Failed to get iterator from file status model for row {}", row)));
    fileStatusModel.get_value(iterator, FileStatusModelColumn::Path as i32).get().
        unwrap_or_else(|| exit(&format!("Failed to get value from file status model for iterator {:?}, column {}",
            iterator, FileStatusModelColumn::Path as i32)))
}