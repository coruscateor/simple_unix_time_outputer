
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use std::time::Duration;

use gtk_estate::adw::glib::clone::Upgrade;

use gtk_estate::{gtk4 as gtk, scs_add, StateContainers, StoredWidgetObject, WidgetAdapter, WidgetStateContainer};

use gtk_estate::corlib::events::SenderEventFunc;

use gtk_estate::corlib::rc_default::RcDefault;

use gtk_estate::gtk4::prelude::{BoxExt, WidgetExt};

use gtk_estate::gtk4::{Box, Orientation, Label, BaselinePosition, Align};

use gtk_estate::adw::{Application, ApplicationWindow, HeaderBar, WindowTitle, prelude::AdwApplicationWindowExt, gtk::prelude::ApplicationWindowExt, gtk::prelude::GtkWindowExt};

use gtk_estate::corlib::{impl_as_any, AsAny};

use gtk_estate::{RcTimeOut, TimeOut};

use time::OffsetDateTime;

use std::any::Any;

pub struct WindowContentState
{

    weak_self: Weak<Self>,
    window_title: WindowTitle,
    hb: HeaderBar,
    unix_time_label: Label,
    internal_content: Box,
    time_out: RcTimeOut<Weak<WindowContentState>>,
    adapted_cbox: Rc<WidgetAdapter<Box, WindowContentState>>

}

impl WindowContentState
{

    pub fn new() -> Rc<Self>
    {

        let cbox = gtk::Box::new(Orientation::Vertical, 0);

        cbox.set_vexpand(true);

        //Add Contents
        
        //HeaderBar

        let window_title = WindowTitle::new("Simple Unix Time Outputer", "");

        let hb = HeaderBar::builder().title_widget(&window_title).build();

        cbox.append(&hb);

        //Internal Content

        let internal_content = gtk::Box::new(Orientation::Vertical, 0);

        //Label

        let unix_time_label = Label::new(Some(""));
        
        internal_content.append(&unix_time_label);

        internal_content.set_vexpand(true);

        internal_content.set_valign(Align::Center);

        cbox.append(&internal_content);

        //Initialise WindowContentState

        let this = Rc::new_cyclic( move |weak_self|
        {

            Self
            {

                weak_self: weak_self.clone(),
                window_title,
                hb,
                unix_time_label,
                internal_content,
                time_out: TimeOut::with_state_ref(Duration::new(1, 0), true, weak_self),
                adapted_cbox: WidgetAdapter::new(&cbox, weak_self)
            }

        });

        //Add WindowContentState to the StateContainers object.

        scs_add!(this);

        let on_timeout: Rc<SenderEventFunc<RcTimeOut<Weak<WindowContentState>>>> = Rc::new(move |sender| {

            if let Some(this) = sender.state().upgrade()
            {

                let utc_now = OffsetDateTime::now_utc();

                let uts = utc_now.unix_timestamp();

                this.unix_time_label.set_label(&uts.to_string());

            }
            else
            {

                return false;

            }

            true

        });

        this.time_out.on_time_out_subscribe(&on_timeout);

        this.time_out.start();

        //Done!

        this

    }

}

impl_as_any!(WindowContentState);

impl WidgetStateContainer for WindowContentState
{

    fn dyn_adapter(&self) -> Rc<dyn StoredWidgetObject>
    {

        self.adapted_cbox.clone()

    }

}
