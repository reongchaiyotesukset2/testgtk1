use gtk::{
    gio,
    glib::{self},   
    prelude::*,
};
use crate::{
    widgets::{Window},
     models::{
        ProvidersModel,
    },
};

mod imp {
 use std::cell::{Cell, RefCell};
  use gtk::subclass::prelude::*;
 use super::*;

  
  #[derive(Default, glib::Properties)]
  #[properties(wrapper_type = super::Application)]
 pub struct Application 
 {
        pub window: RefCell<Option<glib::WeakRef<Window>>>,
        #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
        pub model: ProvidersModel,
        pub lock_timeout_id: RefCell<Option<glib::Source>>,
        #[property(get, set, construct)]
        pub can_be_locked: Cell<bool>,
        #[property(get, set, construct_only)]
        pub is_keyring_open: Cell<bool>,
       
 }
    // Sets up the basics for the GObject
    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type ParentType = gtk::Application;
        type Type = super::Application;
    }
    
    #[glib::derived_properties]
    impl ObjectImpl for Application {}   
    
    impl ApplicationImpl for Application {
       fn startup(&self) {
           //If this part doesn't have the program, it can Error
           self.parent_startup();                       
       }
       fn activate(&self) {
           let app = self.obj();
            println!("activate!!");
           //Show the Window with  widgets::{Window}          
           let window = Window::new(&self.model, &app);
            window.present();
       }
   }
   impl GtkApplicationImpl for Application {}
}
glib::wrapper! {
    pub struct Application(ObjectSubclass<imp::Application>)
        @extends gio::Application, gtk::Application,
        @implements gio::ActionMap, gio::ActionGroup;
}
impl Application {
    pub fn run() -> glib::ExitCode {
       //This code runs as soon as the program starts.
        let app = glib::Object::builder::<Application>() //From now on, all the work will go through the glib library.
            .property("application-id","application.window")
            .property("flags", gio::ApplicationFlags::FLAGS_NONE)
            .build();                  
            app.run()
    }
}

    
  
 
