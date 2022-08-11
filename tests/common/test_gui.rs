use gtk::glib;
use gtk::glib::Cast as _;
use gtk::prelude::GtkWindowExt as _;


pub(crate) struct TestGui
{
    window: gtk::ApplicationWindow
}

impl TestGui
{
    pub fn new(window: gtk::ApplicationWindow) -> Self
    {
        Self{window}
    }

    pub fn findUnstagedChangesView(&self) -> gtk::TreeView
    {
        self.findWidget::<gtk::TreeView>("Unstaged changes view")
    }

    pub fn findStagedChangesView(&self) -> gtk::TreeView
    {
        self.findWidget::<gtk::TreeView>("Staged changes view")
    }

    pub fn findDiffView(&self) -> gtk::TextView
    {
        self.findWidget::<gtk::TextView>("Diff view")
    }

    pub fn findCommitMessageView(&self) -> gtk::TextView
    {
        self.findWidget::<gtk::TextView>("Commit message view")
    }

    pub fn findRefreshButton(&self) -> gtk::Button
    {
        self.findWidget::<gtk::Button>("Refresh button")
    }

    pub fn findCommitButton(&self) -> gtk::Button
    {
        self.findWidget::<gtk::Button>("Commit button")
    }

    pub fn findCommitAmendCheckbox(&self) -> gtk::CheckButton
    {
        self.findWidget::<gtk::CheckButton>("Commit amend checkbox")
    }


    // private

    fn findWidget<T>(&self, name: &str) -> T
        where T: glib::IsA<gtk::Widget>
    {
        gtk_test::find_child_by_name::<T, gtk::ApplicationWindow>(&self.window, name).unwrap()
    }
}

impl Drop for TestGui
{
    fn drop(&mut self)
    {
        for widget in &gtk::Window::list_toplevels() {
            let window = widget.downcast_ref::<gtk::Window>().unwrap();
            window.close();
            while gtk::events_pending() {
                gtk::main_iteration();
            }
        }
    }
}
