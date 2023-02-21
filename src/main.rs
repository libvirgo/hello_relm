pub use relm4::gtk;
pub use relm4::adw;
use relm4::{RelmApp, SimpleComponent, ComponentSender, ComponentParts};
use gtk::{gio};
use gtk::glib::clone;
use adw::prelude::ButtonExt;
use crate::window::WindowView;

mod window;

#[derive(Debug)]
enum AppInput {
    Increment,
    Decrement,
}

struct AppModel {
    counter: u8,
}

struct WindowWidgets {
    label: gtk::Label,
}

impl SimpleComponent for AppModel {
    type Input = AppInput;
    type Output = ();
    type Init = u8;
    type Root = WindowView;
    type Widgets = WindowWidgets;

    fn init_root() -> Self::Root {
        WindowView::default()
    }

    fn init(init: Self::Init, root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = AppModel { counter: 0 };
        let imp = root.imp();
        imp.inc_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Increment);
        }));
        imp.dec_button.connect_clicked(clone!(@strong sender => move |_| {
            sender.input(AppInput::Decrement);
        }));
        imp.label.set_label(&format!("Counter: {}", init));
        let widgets = WindowWidgets { label: imp.label.get() };
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppInput::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
    fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
        widgets.label.set_label(&format!("Counter: {}", self.counter))
    }
}

const APP_ID: &str = "libvirgo.github.io.clash_gtk";

fn main() {
    gio::resources_register_include!("compiled.gresource").expect("failed to register resources.");
    // Create a new application
    let app = adw::Application::builder().application_id(APP_ID).build();
    let relm_app = RelmApp::with_app(app);
    relm_app.run::<AppModel>(0);
    // Connect to "activate" signal of `app`
}
