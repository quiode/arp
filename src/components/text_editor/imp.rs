use gtk::{
    subclass::prelude::*,
    CompositeTemplate,
    glib::{ self, ParamSpec, ParamSpecString },
    prelude::*,
    Label,
};
use once_cell::sync::{ Lazy, OnceCell };
use sourceview5::{ traits::{ BufferExt, ViewExt } };

#[derive(CompositeTemplate, Default)]
#[template(resource = "/com/github/quiode/arp/components/text_editor.ui")]
pub struct TextEditor {
    buffer: OnceCell<sourceview5::Buffer>,
    #[template_child]
    label: TemplateChild<Label>,
}

#[glib::object_subclass]
impl ObjectSubclass for TextEditor {
    const NAME: &'static str = "TextEditor";
    type Type = super::TextEditor;
    type ParentType = gtk::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl TextEditor {}

impl ObjectImpl for TextEditor {
    fn properties() -> &'static [glib::ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::builder("title").build(),
                ParamSpecString::builder("text").build(),
                ParamSpecString::builder("language").build()
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(&self, _id: usize, value: &glib::Value, pspec: &ParamSpec) {
        match pspec.name() {
            "title" =>
                self.label.set_text(
                    &value.get::<String>().expect("Value needs to be of type String")
                ),
            "text" =>
                self.buffer
                    .get()
                    .unwrap()
                    .set_text(&value.get::<String>().expect("Value needs to be of type String")),

            "language" => {
                self.buffer
                    .get()
                    .unwrap()
                    .set_language(
                        Some(
                            &sourceview5::LanguageManager
                                ::default()
                                .language(
                                    &value
                                        .get::<String>()
                                        .expect("Value needs to be of type String")
                                )
                                .expect("Language has to be valid")
                        )
                    )
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _id: usize, pspec: &ParamSpec) -> glib::Value {
        match pspec.name() {
            "title" => self.label.text().to_value(),
            "text" => {
                let buffer = self.buffer.get().unwrap();
                let start = buffer.start_iter();
                let end = buffer.end_iter();
                let text = buffer.text(&start, &end, true).to_value();
                text
            }
            "language" =>
                self.buffer
                    .get()
                    .unwrap()
                    .language()
                    .expect("Language has to be set!")
                    .id()
                    .to_value(),
            _ => unimplemented!(),
        }
    }

    fn constructed(&self) {
        self.parent_constructed();

        let buffer = sourceview5::Buffer::new(None);
        buffer.set_highlight_syntax(true);
        let style = sourceview5::StyleSchemeManager::default().scheme("Adwaita-dark").unwrap();
        buffer.set_style_scheme(Some(&style));
        buffer.set_highlight_matching_brackets(true);

        let view = sourceview5::View::with_buffer(&buffer);
        self.buffer.set(buffer).expect("Buffer can only be set once!");
        view.set_monospace(true);
        view.set_background_pattern(sourceview5::BackgroundPatternType::None);
        view.set_show_line_numbers(true);
        view.set_highlight_current_line(true);
        view.set_tab_width(4);
        view.set_hexpand(true);

        self.obj().append(&view);
    }
}

impl WidgetImpl for TextEditor {}

impl BoxImpl for TextEditor {}