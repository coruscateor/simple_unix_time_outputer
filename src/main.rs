mod applicaion_state;

use gtk_estate::adw::{prelude::*, Application};

use crate::applicaion_state::ApplicationState;

mod window_content_state;

fn main()
{

    //Create an Application object like usual (for GTK/Adwaita programmes oriented around Applications).

    let app = Application::builder().application_id("org.unit_time_gui").build();  

    //Setup the application state.
    
    ApplicationState::new(&app);

    //Run the application

    let run_res = app.run();

    println!("Application ran exiting with code: {}", run_res.value());

}
