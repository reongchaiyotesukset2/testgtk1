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
    #[template(file  = "../../data/resources/ui/window1.ui")] 
    
    #[properties(wrapper_type = super::Window)]
    pub struct Window {  
        //It's required, or you'll get an error.
        #[property(get, set, construct)]
        pub is_locked: Cell<bool>,
       
      
    }
    
    #[glib::object_subclass]
    impl ObjectSubclass for Window {
        const NAME: &'static str = "window_main";
        type Type = super::Window;
        type ParentType = gtk::ApplicationWindow;
        type Interfaces = (gio::Initable,);  
         
            fn class_init(klass: &mut Self::Class) {
               Self::bind_template(klass);   
               //println!("{:#?}",klass);
               
               //let action_group = gio::SimpleActionGroup::new();
               let btn_click_action = gio::SimpleAction::new("btn_click", None);
               
                btn_click_action.connect_activate(move |win,_| {
                let imp = win.imp();
                println!("hello");
            
               });
               
                //klass.add_action(&btn_click_action);
                
            }
            fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
                obj.init_template();
            }   
    }
    
    // sub class 2 ObjectImpl
   //It's okay if it's not there,
    #[glib::derived_properties]
    impl ObjectImpl for Window {}
    impl WidgetImpl for Window {}
    impl WindowImpl for Window {}
    impl ApplicationWindowImpl for Window {}
    impl InitableImpl for Window {    
      fn init(&self, _cancellable: Option<&gio::Cancellable>) -> Result<(), glib::Error> {
          Ok(())
      }      
    }
    impl BinImpl  for Window {}
    //impl BoxImpl for Window{}
    impl ContainerImpl for Window {}

}
// end line models main



//macro glib wrapper
glib::wrapper! {
    pub struct Window(ObjectSubclass<imp::Window>)
        @extends gtk::Widget, gtk::Window,gtk::ApplicationWindow, 
        @implements gio::Initable;
}

// implements main
impl Window {
   pub fn new(_model: &ProvidersModel, app: &Application) -> Self {
        gio::Initable::builder()
            .property("application", app)
            .build(gio::Cancellable::NONE)
            .unwrap()
    }
}
