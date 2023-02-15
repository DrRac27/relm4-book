// ANCHOR: all
use gtk::prelude::{BoxExt, ButtonExt, GtkWindowExt, OrientableExt};
use relm4::{
    gtk, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt, SimpleComponent, WidgetTemplate,
};

// ANCHOR: box_template
#[relm4::widget_template]
impl WidgetTemplate for MyBox {
    view! {
        gtk::Box {
            set_margin_all: 10,
            // Make the boxes visible
            inline_css: "border: 2px solid blue",
        }
    }
}
// ANCHOR_END: box_template

// ANCHOR: spinner_template
#[relm4::widget_template]
impl WidgetTemplate for MySpinner {
    view! {
        gtk::Spinner {
            set_spinning: true,
        }
    }
}
// ANCHOR_END: spinner_template

// ANCHOR: nested_template
#[relm4::widget_template]
impl WidgetTemplate for CustomBox {
    view! {
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_all: 5,
            set_spacing: 5,

            #[template]
            MyBox {
                #[template]
                MySpinner,

                #[template]
                MyBox {
                    #[template]
                    MySpinner,

                    #[template]
                    MyBox {
                        #[template]
                        MySpinner,

                        // Deeply nested!
                        #[name = "child_label"]
                        gtk::Label {
                            set_label: "This is a test",
                        }
                    }
                }
            }
        }
    }
}
// ANCHOR_END: nested_template

#[derive(Default)]
struct AppModel {
    counter: u8,
}

#[derive(Debug)]
enum AppMsg {
    Increment,
    Decrement,
}

// ANCHOR: component_start
#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = u8;
    type Input = AppMsg;
    type Output = ();

    view! {
        gtk::Window {
            set_title: Some("Widget template"),
            set_default_width: 300,
            set_default_height: 100,

            #[template]
            CustomBox {
                gtk::Button {
                    set_label: "Increment",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Increment);
                    },
                },
                gtk::Button {
                    set_label: "Decrement",
                    connect_clicked[sender] => move |_| {
                        sender.input(AppMsg::Decrement);
                    },
                },
                #[template_child]
                child_label {
                    #[watch]
                    set_label: &format!("Counter: {}", model.counter),
                }
            },
        }
    }
    // ANCHOR_END: component_start

    fn init(
        counter: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self { counter };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: AppMsg, _sender: ComponentSender<Self>) {
        match msg {
            AppMsg::Increment => {
                self.counter = self.counter.wrapping_add(1);
            }
            AppMsg::Decrement => {
                self.counter = self.counter.wrapping_sub(1);
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("relm4.example.widget_template");
    app.run::<AppModel>(0);
}
// ANCHOR_END: all
