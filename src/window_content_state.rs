
use std::rc::{Weak, Rc};

use gtk_estate::{impl_widget_state_container_traits, scs_add, StateContainers, WidgetAdapter, WidgetStateContainer};

use gtk_estate::gtk::prelude::{BoxExt, WidgetExt};

use gtk_estate::gtk::{Box, Orientation, Label, Align};

use gtk_estate::adw::{Application, ApplicationWindow, HeaderBar, WindowTitle};

use gtk_estate::corlib::convert::AsAnyRef;

use gtk_estate::{TimeOut, TimeOutRunType};

use time::OffsetDateTime;

use std::any::Any;

use gtk_estate::{DynWidgetStateContainer, WidgetObject};

use gtk_estate::corlib::WeakSelf;

#[derive(Debug)]
pub struct WindowContentState
{

    unix_time_label: Label,
    time_out: TimeOut<WindowContentState>,
    widget_adapter: Rc<WidgetAdapter<ApplicationWindow, Self>>,

}

impl WindowContentState
{

    pub fn new(application: &Application) -> Rc<Self>
    {

        //Initialise The window content box.

        let cbox = Box::new(Orientation::Vertical, 0);

        cbox.set_vexpand(true);
        
        //HeaderBar

        let window_title = WindowTitle::new("Simple Unix Time Outputer", "");

        let hb = HeaderBar::builder().title_widget(&window_title).build();

        cbox.append(&hb);

        //Internal Content

        let internal_content = Box::new(Orientation::Vertical, 0);

        //The Unix time display Label.

        let unix_time_label = Label::new(Some(""));
        
        internal_content.append(&unix_time_label);

        internal_content.set_vexpand(true);

        internal_content.set_valign(Align::Center);

        cbox.append(&internal_content);

        //Initialise ApplicationWindow

        let builder = ApplicationWindow::builder();

        let window = builder.application(application)
            .default_width(1000)
            .default_height(1000)

            //Set the content of the ApplicationWindow.

            .content(&cbox)
            .visible(true)
            .build();

        //Initialise WindowContentState

        let this = Rc::new_cyclic( move |weak_self|
        {

            Self
            {

                unix_time_label,
                time_out: TimeOut::new(TimeOutRunType::Seconds(1), weak_self),
                widget_adapter: WidgetAdapter::new(&window, weak_self)

            }

        });

        //Add WindowContentState to the StateContainers object.

        scs_add!(this);

        //Setup the on_timeout closure.

        let on_timeout = Rc::new(move |this: Rc<Self>|
        {

            let utc_now = OffsetDateTime::now_utc();

            let uts = utc_now.unix_timestamp();

            this.unix_time_label.set_label(&uts.to_string());

            true

        });

        //Set the closure and start the TimeOut.

        this.time_out.set_time_out_fn(&on_timeout);

        this.time_out.start();

        this

    }

}

impl_widget_state_container_traits!(ApplicationWindow, WindowContentState);
