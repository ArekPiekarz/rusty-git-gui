use crate::diff_maker::{DiffMaker, StagedDiffMaker, UnstagedDiffMaker};
use crate::error_handling::{exit, formatFail};
use crate::gui_actions::{
    changeStagingState,
    commitStagedChanges,
    convertFileStatusToStaged,
    convertFileStatusToUnstaged,
    handleChangedFileViewSelection,
    updateCommitButton};
use crate::gui_definitions::{
    FileChangesColumn,
    StagingSwitchStores};
use crate::gui_utils::getBuffer;
use crate::repository::{FileInfo, Repository};

use gtk::{
    BuilderExtManual as _,
    ButtonExt as _,
    CellLayoutExt as _,
    GtkListStoreExt as _,
    GtkListStoreExtManual as _,
    TextBufferExt as _,
    TreeModelExt as _,
    TreeSelectionExt as _,
    TreeViewExt as _,
    WidgetExt as _};
use std::ops::Deref;
use std::rc::Rc;


pub type ApplicationWindow = GuiElement<gtk::ApplicationWindow>;
pub type FileChangesView = GuiElement<gtk::TreeView>;
pub type UnstagedChangesView = FileChangesView;
pub type StagedChangesView = FileChangesView;
pub type TextView = GuiElement<gtk::TextView>;
pub type DiffView = TextView;
pub type CommitMessageView = TextView;
pub type CommitButton = GuiElement<gtk::Button>;
pub type FileChangesStore = GuiElement<gtk::ListStore>;
type UnstagedChangesStore = FileChangesStore;
type StagedChangesStore = FileChangesStore;

pub struct Gui
{
    pub mainWindow: ApplicationWindow,
    pub unstagedChangesView: Rc<UnstagedChangesView>,
    pub stagedChangesView: Rc<StagedChangesView>,
    pub diffView: Rc<DiffView>,
    pub commitMessageView: Rc<CommitMessageView>,
    pub commitButton: Rc<CommitButton>,
    unstagedChangesStore: Rc<UnstagedChangesStore>,
    stagedChangesStore: Rc<StagedChangesStore>
}

impl Gui
{
    pub fn show(&self)
    {
        self.mainWindow.show_all();
    }
}

#[derive(Clone)]
struct FileStatusModels
{
    unstaged: Rc<gtk::ListStore>,
    staged: Rc<gtk::ListStore>
}


const EXPAND_IN_LAYOUT : bool = true;
const PROPAGATE_SIGNAL_TO_DEFAULT_HANDLER : gtk::Inhibit = gtk::Inhibit(true);


pub fn makeGui(repository: Rc<Repository>) -> Gui
{
    let gui = buildGuiFromXml();
    setupMainWindow(
        &gui.mainWindow);
    setupFileChangesStores(
        &gui.unstagedChangesStore,
        &gui.stagedChangesStore,
        &repository);
    setupUnstagedChangesView(
        &gui.unstagedChangesView,
        gui.unstagedChangesStore.clone(),
        gui.stagedChangesStore.clone(),
        repository.clone());
    setupStagedChangesView(
        &*gui.stagedChangesView,
        gui.unstagedChangesStore.clone(),
        gui.stagedChangesStore.clone(),
        repository.clone());
    setupFileChangeViewsInteractions(
        gui.unstagedChangesView.clone(),
        gui.stagedChangesView.clone(),
        gui.diffView.clone(),
        repository.clone());
    setupCommitButton(
        gui.commitButton.clone(),
        gui.commitMessageView.clone(),
        repository,
        gui.stagedChangesStore.clone(),
        &gui.stagedChangesView);

    gui
}

fn buildGuiFromXml() -> Gui
{
    let xml = include_str!("main_window.glade");
    let builder = gtk::Builder::new_from_string(xml);
    Gui{
        mainWindow: makeGuiElement::<gtk::ApplicationWindow>("Main window", &builder),
        unstagedChangesView: Rc::new(makeGuiElement::<gtk::TreeView>("Unstaged changes view", &builder)),
        stagedChangesView: Rc::new(makeGuiElement::<gtk::TreeView>("Staged changes view", &builder)),
        diffView: Rc::new(makeGuiElement::<gtk::TextView>("Diff view", &builder)),
        commitMessageView: Rc::new(makeGuiElement::<gtk::TextView>("Commit message view", &builder)),
        commitButton: Rc::new(makeGuiElement::<gtk::Button>("Commit button", &builder)),
        unstagedChangesStore: Rc::new(makeGuiElement::<gtk::ListStore>("Unstaged changes store", &builder)),
        stagedChangesStore: Rc::new(makeGuiElement::<gtk::ListStore>("Staged changes store", &builder))
    }
}

pub struct GuiElement<T>
{
    object: T,
    name: &'static str
}

impl<T> GuiElement<T>
{
    fn new(object: T, name: &'static str) -> Self
    {
        Self{object, name}
    }

    pub fn name(&self) -> &str
    {
        self.name
    }
}

impl<T> Deref for GuiElement<T>
{
    type Target = T;

    fn deref(&self) -> &Self::Target
    {
        &self.object
    }
}

fn makeGuiElement<T: gtk::IsA<gtk::Object>>
    (name: &'static str, builder: &gtk::Builder) -> GuiElement<T>
{
    let object = builder.get_object::<T>(name)
        .unwrap_or_else(|| exit(&format!("Failed to get GTK object named {} from XML.", name)));
    GuiElement::new(object, name)
}

fn setupMainWindow(mainWindow: &gtk::ApplicationWindow)
{
    mainWindow.connect_delete_event(|_window, _event| {
        gtk::main_quit();
        PROPAGATE_SIGNAL_TO_DEFAULT_HANDLER });
}

fn setupFileChangesStores(
    unstagedChangesStore: &gtk::ListStore,
    stagedChangesStore: &gtk::ListStore,
    repository: &Repository)
{
    let fileInfos = repository.collectFileInfos();
    fillFileChangesStore(unstagedChangesStore, &fileInfos.unstaged);
    fillFileChangesStore(stagedChangesStore, &fileInfos.staged);
}

fn fillFileChangesStore(store: &gtk::ListStore, fileInfos: &[FileInfo])
{
    let fileInfosForStore = fileInfos.iter().map(
        |fileInfo| [&fileInfo.status as &dyn gtk::ToValue, &fileInfo.path as &dyn gtk::ToValue]).collect::<Vec<_>>();

    for fileInfo in fileInfosForStore {
        store.set(&store.append(), &FileChangesColumn::asArrayOfU32(), &fileInfo);
    };
}

fn setupUnstagedChangesView(
    view: &gtk::TreeView,
    unstagedChangesStore: Rc<UnstagedChangesStore>,
    stagedChangesStore: Rc<StagedChangesStore>,
    repository: Rc<Repository>)
{
    setupFilesChangesView(
        view,
        StagingSwitchStores {
            source: unstagedChangesStore,
            target: stagedChangesStore},
        move |path| repository.stageFile(path),
        |fileStatus| convertFileStatusToStaged(&fileStatus));
}

fn setupStagedChangesView(
    view: &gtk::TreeView,
    unstagedChangesStore: Rc<UnstagedChangesStore>,
    stagedChangesStore: Rc<StagedChangesStore>,
    repository: Rc<Repository>)
{
    setupFilesChangesView(
        view,
        StagingSwitchStores {
            source: stagedChangesStore,
            target: unstagedChangesStore},
        move |path| repository.unstageFile(path),
        |fileStatus| convertFileStatusToUnstaged(&fileStatus));
}

fn setupFilesChangesView(
    view: &gtk::TreeView,
    models: StagingSwitchStores,
    switchStagingOfFileInRepository: impl Fn(&str) + 'static,
    convertFileStatusAfterStagingSwitch: impl Fn(&str) -> String + 'static)
{
    FileChangesColumn::asArrayOfI32().iter().for_each(|i| setupColumn(*i, &view));

    view.connect_row_activated(move |_view, row, _column|
        changeStagingState(&models, row, &switchStagingOfFileInRepository, &convertFileStatusAfterStagingSwitch));
}

fn setupColumn(columnIndex: i32, view: &gtk::TreeView)
{
    let renderer = gtk::CellRendererText::new();
    let column = view.get_column(columnIndex)
        .unwrap_or_else(|| exit(&format!("Failed to get column with index {}", columnIndex)));
    column.pack_start(&renderer, EXPAND_IN_LAYOUT);
    column.add_attribute(&renderer, "text", columnIndex);
}

fn setupCommitButton(
    commitButton: Rc<CommitButton>,
    commitMessageView: Rc<TextView>,
    repository: Rc<Repository>,
    stagedChangesStore: Rc<StagedChangesStore>,
    stagedChangesView: &gtk::TreeView)
{
    let commitMessageBuffer = Rc::new(getBuffer(&commitMessageView)
        .unwrap_or_else(
            |_e| exit("Failed to setup commit button updater, because commit message view does not have a buffer.")));

    commitButton.connect_clicked(
        move |_button| commitStagedChanges(&commitMessageView, &repository, &stagedChangesStore)
            .unwrap_or_else(|e| exit(&formatFail(&e))));

    let stagedChangesModel = stagedChangesView.get_model()
        .unwrap_or_else(
            || exit("Failed to setup commit button updater, because staged files view does not have a model."));

    updateCommitButton(&commitButton, &stagedChangesModel, &commitMessageBuffer);

    let commitButton2 = commitButton.clone();
    let commitButton3 = commitButton.clone();
    let commitMessageBuffer2 = commitMessageBuffer.clone();
    let commitMessageBuffer3 = commitMessageBuffer.clone();

    stagedChangesModel.connect_row_inserted(
        move |model, _row, _iter| updateCommitButton(&commitButton, model, &commitMessageBuffer));
    stagedChangesModel.connect_row_deleted(
        move |model, _row| updateCommitButton(&commitButton2, model, &commitMessageBuffer2));
    commitMessageBuffer3.connect_changed(
        move |buffer| updateCommitButton(&commitButton3, &stagedChangesModel, buffer));
}

fn setupFileChangeViewsInteractions(
    unstagedFilesView: Rc<UnstagedChangesView>,
    stagedFilesView: Rc<StagedChangesView>,
    diffView: Rc<TextView>,
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
        &stagedFilesView,
        diffView,
        StagedDiffMaker{repository},
        unstagedFilesViewToUnselect);
}

fn connectSelectionChanged(
    filesView: &gtk::TreeView,
    diffView: Rc<TextView>,
    diffMaker: impl DiffMaker + 'static,
    filesViewToUnselect: Rc<FileChangesView>)
{
    filesView.get_selection().connect_changed(
        move |selection| handleChangedFileViewSelection(selection, &diffView, &diffMaker, &filesViewToUnselect)
            .unwrap_or_else(|e| exit(&formatFail(&e))));
}