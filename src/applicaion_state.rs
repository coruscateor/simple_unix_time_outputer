use gtk_estate::corlib::impl_weak_self_trait;
use gtk_estate::gtk::prelude::ApplicationExt;

//use gtk_estate::{gtk4 as gtk, scs_set_app, ApplicationAdapter, ApplicationStateContainer, StateContainers, StoredApplicationObject};

use gtk_estate::adw::Application;

use gtk_estate::scs_set_application_state;

use std::{rc::*, any::Any};

use std::cell::{RefCell, Ref, RefMut};

use gtk_estate::StateContainers;

//use gtk_estate::corlib::{AsAny, impl_as_any};

//use gtk_estate::AdwApplcationWindowState;

use crate::window_content_state::WindowContentState;

use gtk_estate::corlib::WeakSelf;

pub struct ApplicationState
{

    app: Application,
    weak_self: Weak<ApplicationState>

    //app_ad: Rc<ApplicationAdapter<Application, ApplicationState>>

}

impl ApplicationState
{

    //Missing parent

    pub fn new(app: &Application) -> Rc<ApplicationState>
    {

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                app: app.clone(),
                weak_self: weak_self.clone()

                //app_ad: ApplicationAdapter::new(app, weak_self)

            }

        });

        app.connect_activate(move |app|
        {

            //new window

            WindowContentState::new(app);

            /*
            let content = WindowContentState::new(app);

            AdwApplcationWindowState::builder_with_content_visible(|builder| {

                builder.application(app)
                .default_width(1000)
                .default_height(1000)
                .build()

            }, &content);
            */

        });

        //Set the application state

        scs_set_application_state!(this);

        //scs_set_app!(this);

        this

    }

    pub fn app_ref(&self) -> &Application
    {

        &self.app

    }

    /*
    fn adapter(&self) -> &ApplicationAdapter<Application, ApplicationState>
    {
        
        &self.app_ad

    }
    */

}

impl_weak_self_trait!(ApplicationState);

/*
impl_as_any!(ApplicationState);

impl ApplicationStateContainer for ApplicationState
{

    fn dyn_adapter(&self) -> Rc<dyn StoredApplicationObject>
    {

        self.app_ad.clone()

    }

}
*/

