use gtk::{
    gio,
    glib::{self,clone},
    subclass::prelude::*,
    glib::Properties,
    prelude::*,
};

use crate::{
    widgets::{Window,PreferencesWindow},
     models::{ProvidersModel},
};
use std::rc::Rc;

mod imp {
  use std::cell::{Cell, RefCell};
  use super::*;
  #[derive(Default,glib::Properties,Debug)]
  #[properties(wrapper_type = super::Application)]
 pub struct Application {
        pub window: RefCell<Option<glib::WeakRef<Window>>>,
        pub window2 : glib::WeakRef<Window>,
        pub model: ProvidersModel,      
        #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
      
 }
    // Sets up the basics for the GObject
    #[glib::object_subclass]
    impl ObjectSubclass for Application {
        const NAME: &'static str = "Application";
        type Type = super::Application;
        type ParentType = gtk::Application;
        
    }
    
    #[glib::derived_properties]
    impl ObjectImpl for Application {}   
    
    impl ApplicationImpl for Application {

       fn startup(&self) {
           //If this part doesn't have the program, it can Error
           self.parent_startup();   
           let app = self.obj();  
               
                 let quit_action = gio::ActionEntry::builder("quit")
                .activate(|app: &Self::Type, _, _| {
                   
                      app.quit()
                }).build();      
                
                //preferences test
                
                 let preferences_action = gio::ActionEntry::builder("preferences")
                .activate(|app: &Self::Type, _, _| {
                 let preferences = PreferencesWindow::default();
                     println!("preferences");
                }).build();
                
                
                // About
            let about_action = gio::ActionEntry::builder("about")
                .activate(|app: &Self::Type, _, _| {
                    let window = app.active_window();
                    gtk::AboutDialog::builder()                      
                        .website("https://gitlab.gnome.org/World/Authenticator")                
                        .artists(vec!["Alexandros Felekidis", "Tobias Bernard"])
                        .license_type(gtk::License::Gpl30)
                        .build()
                        .present();
                })
                .build();
                
             app.add_action_entries([
                quit_action,
                preferences_action,
                about_action,
            ]);
            
             let preferences_action = app.lookup_action("preferences").unwrap();
             app.bind_property("is-locked", &preferences_action, "enabled")
                .invert_boolean()
                .sync_create()
                .build();
       }
       fn activate(&self) {
           let app = self.obj();
           //let window = app.active_window();
           //println!("window::{:#?}",window);
            
            app.set_accels_for_action("app.quit", &["<primary>q"]);
            app.set_accels_for_action("app.preferences", &["<primary>p"]);
            app.set_accels_for_action("app.about", &["<primary>a"]);
            
           //Show the Window with  widgets::{Window}     
           
           let window = Window::new(&self.model, &app);
            //window.present();
            if let Some(ref win) = *self.window.borrow() {
                let window = win.upgrade().unwrap();
                window.present();
                return;
            }
            
           
            window.present();
            self.window.replace(Some(window.downgrade()));
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
    
   pub fn active_window(&self) -> Window {
        self.imp()
            .window
            .borrow()
            .as_ref()
            .unwrap()
            .upgrade()
            .unwrap()
    }
    
   
}

