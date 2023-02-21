pub use relm4::gtk;
pub use relm4::gtk::glib;
pub use relm4::gtk::gio;
use gtk::{
    subclass::prelude::*,
};
use glib::Object;


mod imp {
    pub use relm4::gtk;
    pub use relm4::adw;
    pub use gtk::glib;
    use gtk::{CompositeTemplate, TemplateChild, Button};
    use glib::subclass::InitializingObject;
    use gtk::Label;
    use adw::{prelude::*, subclass::prelude::*};

    #[derive(CompositeTemplate, Default)]
    #[template(resource = "/libvirgo/github/io/clash_gtk/window.xml")]
    pub struct WindowView {
        #[template_child]
        pub inc_button: TemplateChild<Button>,
        #[template_child]
        pub dec_button: TemplateChild<Button>,
        #[template_child]
        pub label: TemplateChild<Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WindowView {
        const NAME: &'static str = "TodoWindow";
        type Type = super::WindowView;
        type ParentType = gtk::ApplicationWindow;
        fn class_init(_klass: &mut Self::Class) {
            _klass.bind_template();
        }
        fn instance_init(_obj: &InitializingObject<Self>) {
            _obj.init_template();
        }
    }

    impl ObjectImpl for WindowView {
        fn constructed(&self) {
            self.parent_constructed();
        }
    }

    impl WidgetImpl for WindowView {}

    impl WindowImpl for WindowView {}

    impl ApplicationWindowImpl for WindowView {}
}

glib::wrapper! {
    pub struct WindowView(ObjectSubclass<imp::WindowView>)
        @extends gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl Default for WindowView {
    fn default() -> Self {
        Object::new::<Self>(&[])
    }
}

impl WindowView {
    pub fn imp(&self) -> &imp::WindowView {
        imp::WindowView::from_obj(self)
    }
}
