use gtk_estate::corlib::impl_weak_self_trait;

use gtk_estate::gtk::prelude::ApplicationExt;

use gtk_estate::adw::Application;

use gtk_estate::scs_set_application_state;

use std::rc::{Rc, Weak};

use gtk_estate::StateContainers;

use crate::window_state::WindowState;

use gtk_estate::corlib::WeakSelf;

pub struct ApplicationState
{

    app: Application,
    weak_self: Weak<ApplicationState>

}

impl ApplicationState
{

    pub fn new(app: &Application) -> Rc<ApplicationState>
    {

        let this = Rc::new_cyclic(|weak_self|
        {

            Self
            {

                app: app.clone(),
                weak_self: weak_self.clone()

            }

        });

        app.connect_activate(move |app|
        {

            //new window

            WindowState::new(app);
            
        });

        //Set the application state

        scs_set_application_state!(this);

        this

    }

    pub fn app_ref(&self) -> &Application
    {

        &self.app

    }

}

impl_weak_self_trait!(ApplicationState);
