use crate::converters::toI32;
use crate::diff_maker::{DiffMaker, StagedDiffMaker, UnstagedDiffMaker};
use crate::error_handling::{exit, formatFail};
use crate::gui_actions::{
    changeStagingState,
    commitStagedChanges,
    convertFileStatusToStaged,
    convertFileStatusToUnstaged,
    handleChangedFileViewSelection,
    updateCommitButton,
};
use crate::gui_definitions::{FILE_STATUS_MODEL_COLUMN_INDICES, StagingAreaChangeModels};
use crate::gui_utils::getBuffer;
use crate::repository::{FileInfo, Repository};
use gtk::{
    ButtonExt as _,
    CellLayoutExt as _,
    ContainerExt as _,
    GtkListStoreExt as _,
    GtkListStoreExtManual as _,
    GtkWindowExt as _,
    PanedExt as _,
    TextBufferExt as _,
    TreeModelExt as _,
    TreeSelectionExt as _,
    TextViewExt as _,
    TreeViewColumnExt as _,
    TreeViewExt as _,
    WidgetExt as _,
};
use std::rc::Rc;


#[derive(Clone)]
struct FileStatusModels
{
    unstaged: Rc<gtk::ListStore>,
    staged: Rc<gtk::ListStore>
}


const EXPAND_IN_LAYOUT : bool = true;
const SPACING : i32 = 8;
const FILE_STATUS_COLUMN_TYPE : gtk::Type = gtk::Type::String;
const FILE_PATH_COLUMN_TYPE : gtk::Type = gtk::Type::String;


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
    diffVerticalBox.add(&*diffView);

    let commitVerticalBox = gtk::Box::new(gtk::Orientation::Vertical, SPACING);
    diffAndCommitPane.add2(&commitVerticalBox);
    commitVerticalBox.add(&gtk::Label::new("Commit message:"));
    let commitMessageView = makeCommitMessageView();
    commitVerticalBox.add(&*commitMessageView);
    let commitButton = makeCommitButton(
        commitMessageView.clone(), repository.clone(), &commitVerticalBox, fileStatusModels.staged);

    setupFileViews(unstagedFilesStatusView, &stagedFilesStatusView, diffView, repository);
    setupCommitButtonUpdater(commitButton, &*stagedFilesStatusView, &commitMessageView);

    window.show_all();
}

fn makeWindow(gtkApp: &gtk::Application) -> gtk::ApplicationWindow
{
    let window = gtk::ApplicationWindow::new(gtkApp);
    window.set_title("Rusty Git Gui");
    window.set_default_size(400, 400);
    window
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

fn makeUnstagedFilesStatusView(fileStatusModels: FileStatusModels, repository: Rc<Repository>) -> Rc<gtk::TreeView>
{
    makeFilesStatusView(
        "Unstaged files view",
        StagingAreaChangeModels{
            source: fileStatusModels.unstaged,
            target: fileStatusModels.staged},
        move |path| repository.stageFile(path),
        |fileStatus| convertFileStatusToStaged(&fileStatus))
}

fn makeStagedFilesStatusView(fileStatusModels: FileStatusModels, repository: Rc<Repository>) -> Rc<gtk::TreeView>
{
    makeFilesStatusView(
        "Staged files view",
        StagingAreaChangeModels{
            source: fileStatusModels.staged,
            target: fileStatusModels.unstaged},
        move |path| repository.unstageFile(path),
        |fileStatus| convertFileStatusToUnstaged(&fileStatus))
}

fn makeFilesStatusView(
    name: &str,
    models: StagingAreaChangeModels,
    switchStagingOfFileInRepository: impl Fn(&str) + 'static,
    convertFileStatusAfterStagingSwitch: impl Fn(&str) -> String + 'static)
    -> Rc<gtk::TreeView>
{
    let fileStatusView = Rc::new(gtk::TreeView::new_with_model(&*models.source));
    fileStatusView.set_vexpand(true);
    appendColumn("Status", &fileStatusView);
    appendColumn("File", &fileStatusView);
    fileStatusView.connect_row_activated(move |_view, row, _column|
        changeStagingState(&models, row, &switchStagingOfFileInRepository, &convertFileStatusAfterStagingSwitch));
    fileStatusView.set_name(name);
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
    diffView.set_name("Diff view");
    diffView.set_editable(false);
    diffView.set_monospace(true);
    diffView.set_vexpand(true);
    diffView.set_hexpand(true);
    diffView
}

fn makeCommitMessageView() -> Rc<gtk::TextView>
{
    let commitMessageView = Rc::new(gtk::TextView::new());
    commitMessageView.set_name("Commit message view");
    commitMessageView.set_vexpand(true);
    commitMessageView
}

fn makeCommitButton(
    commitMessageView: Rc<gtk::TextView>,
    repository: Rc<Repository>,
    layoutBox: &gtk::Box,
    stagedFilesModel: Rc<gtk::ListStore>)
    -> Rc<gtk::Button>
{
    let commitButton = Rc::new(gtk::Button::new_with_label("Commit"));
    commitButton.set_name("Commit button");
    commitButton.connect_clicked(
        move |_button| commitStagedChanges(&commitMessageView, &repository, &stagedFilesModel)
            .unwrap_or_else(|e| exit(&formatFail(&e))));
    layoutBox.add(&*commitButton);
    commitButton
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
        move |selection| handleChangedFileViewSelection(selection, &diffView, &diffMaker, &filesViewToUnselect)
            .unwrap_or_else(|e| exit(&formatFail(&e))));
}

fn setupCommitButtonUpdater(
    commitButton: Rc<gtk::Button>,
    stagedFilesView: &gtk::TreeView,
    commitMessageView: &gtk::TextView)
{
    let stagedFilesModel = stagedFilesView.get_model()
        .unwrap_or_else(
            || exit("Failed to setup commit button updater, because staged files view does not have a model."));
    let commitMessageBuffer = Rc::new(getBuffer(&commitMessageView)
        .unwrap_or_else(
            |_e| exit("Failed to setup commit button updater, because commit message view does not have a buffer.")));

    updateCommitButton(&commitButton, &stagedFilesModel, &commitMessageBuffer);

    let commitButton2 = commitButton.clone();
    let commitButton3 = commitButton.clone();
    let commitMessageBuffer2 = commitMessageBuffer.clone();
    let commitMessageBuffer3 = commitMessageBuffer.clone();

    stagedFilesModel.connect_row_inserted(
        move |model, _row, _iter| updateCommitButton(&commitButton, model, &commitMessageBuffer));
    stagedFilesModel.connect_row_deleted(
        move |model, _row| updateCommitButton(&commitButton2, model, &commitMessageBuffer2));
    commitMessageBuffer3.connect_changed(
        move |buffer| updateCommitButton(&commitButton3, &stagedFilesModel, buffer));

}