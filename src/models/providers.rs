use gtk::{gio, glib, subclass::prelude::*};
mod imp {


    use super::*;
    
    #[derive(Default)]
   // pub struct ProvidersModel(pub RefCell<Vec<Provider>>, pub Cell<bool>);
   pub struct ProvidersModel{}
     #[glib::object_subclass]
    impl ObjectSubclass for ProvidersModel {
        const NAME: &'static str = "ProvidersModel";
        type Type = super::ProvidersModel;
        //type Interfaces = (gio::ListModel,);
     }          
        impl ObjectImpl for ProvidersModel {}       
    }
     glib::wrapper! {
     pub struct ProvidersModel(ObjectSubclass<imp::ProvidersModel>)
        @implements gio::ListModel;
    }
   impl ProvidersModel {}
   impl Default for ProvidersModel {
    fn default() -> Self {
        glib::Object::new()
    }
   }
