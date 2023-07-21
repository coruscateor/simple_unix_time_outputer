use gtk::prelude::ApplicationExt;
use gtk_estate::gtk4 as gtk;

use gtk_estate::*;

//use gtk::Application;

use gtk_estate::adw::Application;

use std::{rc::*, any::Any};

use std::cell::{RefCell, Ref, RefMut};

//use corlib::rc_self_init_refcell_setup_returned_named;

use corlib::{rc_self_rfc_setup, NonOption}; //use corlib::{rc_self_refcell_setup, NonOption}; //rc_self_init_refcell_setup_returned

use crate::window_state::*;

pub struct ApplicattionState
{

    app: Application,
    weak_self: NonOption<Weak<RefCell<Self>>>

}

impl ApplicattionState
{

    pub fn new(app: Application) -> Rc<RefCell<Self>> //
    {

        let this = Self
        {

            app,
            weak_self: NonOption::invalid()

        };

        //let res = rc_self_init_refcell_setup_returned!(this, weak_self);
        
        let rc_self = Rc::new(RefCell::new(this));

        //rc_self_refcell_setup!(rc_self, weak_self);

        rc_self_rfc_setup!(rc_self, weak_self);

        //

        {

            let b_self = rc_self.borrow();

            /*
            b_self.app.connect_startup(|_|
            {
    
                if let Err(err) = adw::init()
                {

                    println!("Adwaita init errror: {}", err);

                }
    
            });
            */

            b_self.app.connect_activate(move |app|
            {

                //new window

                WindowState::new(app);

            });

        }

        //let sc = StateContainers::get();

        //sc.adw().get_applications_mut().add_refcell(&rc_self);

        //

        let scs = StateContainers::get();

        //add this application

        scs.adw().borrow_mut_applications().add_refcell(&rc_self);

        rc_self

    }

    //get weak self

}

/*
pub fn run()
{

    //let apst = ApplicattionState::new();

    //apst.

}
*/

impl_has_application!(app, ApplicattionState);

/*
impl HasObject<Application> for ApplicattionState
{

    fn get_object(&self) -> &Application
    {
        
        &self.app
        
    }

}
*/
