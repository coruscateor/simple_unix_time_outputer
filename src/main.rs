mod applicaion_state;

mod window_state;

use gtk_estate::{adw::{prelude::*, Application}, StateContainers};

use crate::applicaion_state::ApplicattionState;

mod window_contents_state;

fn main() {

    let app = Application::builder().application_id("org.unit_time_gui").build();

    //This instance of the State containers is needed for its global access to work

    //The static mut vaiable contains a weak reference to the below reference counted StateContainers instance

    //let _sc = StateContainers::new();

    StateContainers::init();    

    //let teas = 
    
    ApplicattionState::new(&app);

    //sc.get_adw_state_ref().get_applications_mut().add_refcell(&teas); //.insert(app.clone(), teas); //.add(&TextEdApplicattionState::new(app.clone())); //&std::rc::Rc<RefCell<(dyn gtk_estate::HasObject<gtk_estate::gtk4::Application>)>>) //&dyn Any)); //.downcast_ref().unwrap() // as &Rc<RefCell<dyn HasObject<Application>>>);

    println!("run unit_time_gui!");

    let run_res = app.run();

    println!("Application ran exiting with code: {}", run_res.value());

}
