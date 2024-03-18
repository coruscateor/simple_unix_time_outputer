use gtk::prelude::ApplicationExt;
use gtk_estate::{gtk4 as gtk, ApplicationAdapter, ApplicationStateContainer, StateContainers, StoredApplicationObject};

//use gtk_estate::*;

//use gtk::Application;

use gtk_estate::adw::Application;

use std::{rc::*, any::Any};

use std::cell::{RefCell, Ref, RefMut};

//use corlib::rc_self_init_refcell_setup_returned_named;

use gtk_estate::corlib::{rc_self_rfc_setup, NonOption}; //use corlib::{rc_self_refcell_setup, NonOption}; //rc_self_init_refcell_setup_returned

//use crate::window_state::*;

use gtk_estate::corlib::{AsAny, impl_as_any};

use gtk_estate::AdwApplcationWindowState;

pub struct ApplicationState
{

    app_ad: ApplicationAdapter<Application>,
    //weak_self: NonOption<Weak<RefCell<Self>>>
    weak_self: Weak<ApplicationState>

}

impl ApplicationState
{

    //Missing parent

    pub fn new(app: &Application) -> Rc<ApplicationState>
    {

        let this = Rc::new_cyclic(|weak_self|
        {

            //let asc;

            //{

                //let any_this: &dyn Any = weak_self; //.as_any();
                
                //weak_self.downcast

                //asc = any_this.downcast_ref::<Weak<dyn ApplicationStateContainer>>().expect("Error: No Rc<dyn ApplicationStateContainer>").clone();

            //}

            //let my_self = weak_self.upgrade(); //.unwrap();

            //let any_this: &dyn Any = weak_self.as_any();

            let any_this: &dyn Any = weak_self; //.as_any();
                
            //weak_self.downcast

            let asc = any_this.downcast_ref::<Weak<dyn ApplicationStateContainer>>().expect("Error: No Weak<dyn ApplicationStateContainer>");

            Self
            {

                app_ad: ApplicationAdapter::new(&app, &asc),
                weak_self: weak_self.clone()

            }

        });

        app.connect_activate(move |app|
        {

            //new window

            //WindowState::new(app);

            AdwApplcationWindowState::builder(|builder| {

                builder.application(app)
                .default_width(1000)
                .default_height(1000)
                //.title("Rustpad")
                //.show_menubar(true)
                //.content(&contents)
                .build()

            });

        });

        /*
        let this = Self
        {

            app,
            weak_self: NonOption::invalid()

        };
        */

        //let res = rc_self_init_refcell_setup_returned!(this, weak_self);
        
        //let rc_self = Rc::new(RefCell::new(this));

        //rc_self_refcell_setup!(rc_self, weak_self);

        //rc_self_rfc_setup!(rc_self, weak_self);

        //

        //{

            //let b_self = rc_self.borrow();

            /*
            b_self.app.connect_startup(|_|
            {
    
                if let Err(err) = adw::init()
                {

                    println!("Adwaita init errror: {}", err);

                }
    
            });
            */

            //b_self.app.connect_activate(move |app|
            //{

                //new window

                //WindowState::new(app);

           // });

        //}

        //let sc = StateContainers::get();

        //sc.adw().get_applications_mut().add_refcell(&rc_self);

        //

        let scs = StateContainers::get();

        //add this application

        let any_this: &dyn Any = &this;

        let asc = any_this.downcast_ref::<Rc<dyn ApplicationStateContainer>>().expect("Error: No Rc<dyn ApplicationStateContainer>");

        scs.set_application_state_or_panic(asc);

        //scs.set_application_state(state) //.adw().borrow_mut_applications().add_refcell(&rc_self);

        this

    }

    //get weak self

    fn adapter(&self) -> &ApplicationAdapter<Application>
    {
        
        &self.app_ad

    }

}

impl_as_any!(ApplicationState);

impl ApplicationStateContainer for ApplicationState
{

    fn adapted_application(&self) -> &(dyn StoredApplicationObject)
    {
        
        &self.app_ad

    }

}

/*
pub fn run()
{

    //let apst = ApplicattionState::new();

    //apst.

}
*/

//impl_has_application!(app, ApplicattionState);

/*
impl HasObject<Application> for ApplicattionState
{

    fn get_object(&self) -> &Application
    {
        
        &self.app
        
    }

}
*/
