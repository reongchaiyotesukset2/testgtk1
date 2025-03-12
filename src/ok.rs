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
    use glib::subclass;
   
    use super::*;
    
    #[derive(Default, gtk::CompositeTemplate, glib::Properties)]
    #[template(file  = "./window.ui")]
    #[properties(wrapper_type = super::Window)]
    pub struct Window {  
        #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
        #[template_child]
        pub btnclick: TemplateChild<gtk::Button>,
    }
    
    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "Window";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;
        type Interfaces = (gio::Initable,);
        
        //first function on ObjectSubclass
        //function about call action
        fn class_init(klass: &mut Self::Class) {
            
             Self::bind_template(klass);
            
        }
        // function 2
        fn instance_init(obj: &subclass::InitializingObject<Self>) {
            obj.init_template();
        } 
    }
    // sub class 2 ObjectImpl
    #[glib::derived_properties]
    impl ObjectImpl for Window {
       fn constructed(&self) {
           self.parent_constructed();           
       }
    }
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl InitableImpl for Window {
      fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
          Ok(())
      }
    }
    impl BinImpl  for Window {}
    impl ContainerImpl for Window {}

       
}
// end line models main



//macro glib wrapper
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, 
        @implements gio::Initable;
}
// implements main
impl Window {
        pub fn new(model: &ProvidersModel, app: &Application) -> Self {
        gio::Initable::builder()
            .property("application", app)           
            .build(gio::Cancellable::NONE)
            .unwrap()
    }

 
    pub fn empty_status_page()
    {
    
    }
}
// endline implements main
