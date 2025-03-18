
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use std::time::Duration;

use gtk_estate::adw::glib::clone::Upgrade;

use gtk_estate::{/*gtk4 as gtk,*/ impl_widget_state_container_traits, scs_add, StateContainers, WidgetAdapter, WidgetStateContainer};

use gtk_estate::corlib::events::SenderEventFunc;

//use gtk_estate::corlib::rc_default::RcDefault;

use gtk_estate::gtk::prelude::{BoxExt, WidgetExt};

use gtk_estate::gtk::{Box, Orientation, Label, BaselinePosition, Align};

use gtk_estate::adw::{Application, ApplicationWindow, HeaderBar, WindowTitle, prelude::AdwApplicationWindowExt, gtk::prelude::ApplicationWindowExt, gtk::prelude::GtkWindowExt};

use gtk_estate::corlib::convert::AsAnyRef;

use gtk_estate::corlib::impl_as_any_ref;

use gtk_estate::{TimeOut, TimeOutRunType};

use time::OffsetDateTime;

use std::any::Any;

use gtk_estate::{DynWidgetStateContainer, WidgetObject};

use gtk_estate::corlib::WeakSelf;

#[derive(Debug)]
pub struct WindowContentState
{

    //weak_self: Weak<Self>,
    //window_title: WindowTitle,
    //hb: HeaderBar,
    unix_time_label: Label,
    //internal_content: Box,
    time_out: TimeOut<WindowContentState>, //Weak<WindowContentState>>,
    //adapted_cbox: Rc<WidgetAdapter<Box, WindowContentState>>
    //adapted_window: Rc<WidgetAdapter<ApplicationWindow, Self>>,
    widget_adapter: Rc<WidgetAdapter<ApplicationWindow, Self>>,

}

impl WindowContentState
{

    pub fn new(application: &Application) -> Rc<Self>
    {

        let cbox = Box::new(Orientation::Vertical, 0);

        cbox.set_vexpand(true);

        //Add Contents
        
        //HeaderBar

        let window_title = WindowTitle::new("Simple Unix Time Outputer", "");

        let hb = HeaderBar::builder().title_widget(&window_title).build();

        cbox.append(&hb);

        //Internal Content

        let internal_content = Box::new(Orientation::Vertical, 0);

        //Label

        let unix_time_label = Label::new(Some(""));
        
        internal_content.append(&unix_time_label);

        internal_content.set_vexpand(true);

        internal_content.set_valign(Align::Center);

        cbox.append(&internal_content);

        //ApplicationWindow

        let builder = ApplicationWindow::builder();

        let window = builder.application(application)
            .default_width(1000)
            .default_height(1000)

            //Make sure to set the content of the ApplicationWindow.

            .content(&cbox)
            .visible(true)
            //.hide_on_close(false)
            .build();

        //Initialise WindowContentState

        let this = Rc::new_cyclic( move |weak_self|
        {

            Self
            {

                //weak_self: weak_self.clone(),
                //window_title,
                //hb,
                unix_time_label,
                //internal_content,
                time_out: TimeOut::new(TimeOutRunType::Seconds(1), weak_self),
                //adapted_window: WidgetAdapter::new(&window, weak_self)
                widget_adapter: WidgetAdapter::new(&window, weak_self)

            }

        });

        //Add WindowContentState to the StateContainers object.

        scs_add!(this);

        let on_timeout = Rc::new(move |this: Rc<Self>| { //: Rc<SenderEventFunc<TimeOut<Weak<WindowContentState>>>>

            let utc_now = OffsetDateTime::now_utc();

            let uts = utc_now.unix_timestamp();

            this.unix_time_label.set_label(&uts.to_string());

            true

        });

        this.time_out.set_time_out_fn(&on_timeout);

        this.time_out.start();

        //Done!

        this

    }

}

impl_widget_state_container_traits!(ApplicationWindow, WindowContentState);

/*
impl_as_any!(WindowContentState);

impl WidgetStateContainer for WindowContentState
{

    fn dyn_adapter(&self) -> Rc<dyn StoredWidgetObject>
    {

        self.adapted_cbox.clone()

    }

}
*/