//call library dependency gtk glib 
use std::cell::Cell;
use gtk::{
    gio,
    glib::{self},
    subclass::prelude::*,
    prelude::*,
};
//call file 
use crate::{
    application::Application, // file application.rs
    models::{ ProvidersModel}, // folder models call with mod.rs
};
//models main
mod imp {
	
    //use gtk::subclass::prelude::*;
    //use glib::subclass;
    use super::*;
    
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(file  = "../../../data/resources/ui/preferences.ui")] 
    
    #[properties(wrapper_type = super::PreferencesWindow)]
    pub struct PreferencesWindow {  
        //It's required, or you'll get an error.
        #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
       
        #[template_child]
        pub btn_click1: TemplateChild<gtk::Button>,
         #[template_child]
        pub btn_click2: TemplateChild<gtk::Button>,
         #[template_child]
        pub btn_click3: TemplateChild<gtk::Button>,
        pub test : String, //test make name
    }
    
    #[glib::object_subclass]
    impl ObjectSubclass for PreferencesWindow {
        const NAME: &'static str = "PreferencesWindow";
        type Type = super::PreferencesWindow;
        type ParentType = gtk::ApplicationWindow;
        type Interfaces = (gio::Initable,);  
         
            fn class_init(klass: &mut Self::Class) {
               Self::bind_template(klass);       
               
               
            }
            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                obj.init_template();
            }   
    }
    
    // sub class 2 ObjectImpl
   //It's okay if it's not there,
    #[glib::derived_properties]
    impl ObjectImpl for PreferencesWindow {}
    impl WidgetImpl for PreferencesWindow {}
    impl WindowImpl for PreferencesWindow {}
    impl ApplicationWindowImpl for PreferencesWindow {}
    impl InitableImpl for PreferencesWindow {    
      fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
          Ok(())
      }      
    }
    impl BinImpl  for PreferencesWindow {}
    //impl BoxImpl for PreferencesWindow{}
    impl ContainerImpl for PreferencesWindow {}

}
// end line models main



//macro glib wrapper
glib::wrapper! {
    pub struct PreferencesWindow(ObjectSubclass<imp::PreferencesWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, 
        @implements gio::Initable;
}

// implements main
impl PreferencesWindow {
   pub fn new(_model: &ProvidersModel, app: &Application) -> Self {
        gio::Initable::builder()
            .property("application", app)
            .build(gio::Cancellable::NONE)
            .unwrap()
    }
}
impl Default for PreferencesWindow {
    fn default() -> Self {
        glib::Object::new()
    }
}
