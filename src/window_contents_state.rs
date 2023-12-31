
use std::cell::RefCell;
use std::rc::{Weak, Rc};
use std::time::Duration;

use corlib::events::SenderEventFunc;
use corlib::rc_default::RcDefault;
use gtk_estate::gtk4::traits::{BoxExt, WidgetExt};
use gtk_estate::{HasObject, impl_has_box, impl_has_object, StateContainers}; //get_state_containers, 

use gtk_estate::gtk4::{self as gtk, Box, Orientation, Label, BaselinePosition, Align};

use gtk_estate::adw::{Application, ApplicationWindow, HeaderBar, WindowTitle, prelude::AdwApplicationWindowExt, gtk::prelude::ApplicationWindowExt, gtk::prelude::GtkWindowExt};

use gtk_estate::corlib::{NonOption, rc_self_setup};

use gtk_estate::time_out::*;

use time::OffsetDateTime;

pub struct WindowContentsState
{

    weak_self: RefCell<NonOption<Weak<Self>>>,
    cbox: Box,
    window_title: WindowTitle,
    hb: HeaderBar,
    unix_time_label: Label,
    internal_content: Box,
    time_out: RcTimeOut

}

impl WindowContentsState
{

    pub fn new(app: &ApplicationWindow) -> Rc<Self>
    {

        let cbox = Box::new(Orientation::Vertical, 0);

        cbox.set_vexpand(true);

        //cbox.set_baseline_position(BaselinePosition::Center);

        //cbox.set_valign(Align::Center);

        //Add Contents
        
        //HeaderBar

        let window_title = WindowTitle::new("Simple Unix Time Outputer", "");

        let hb = HeaderBar::builder().title_widget(&window_title).build();

        cbox.append(&hb);

        //internal_content

        let internal_content = Box::new(Orientation::Vertical, 0);

        //Label

        let unix_time_label = Label::new(Some("unix_time_label"));

        internal_content.append(&unix_time_label);

        internal_content.set_vexpand(true);

        internal_content.set_valign(Align::Center);

        //unix_time_label.set_valign(Align::Center);

        cbox.append(&internal_content);

        let time_out = TimeOut::new(Duration::new(1, 0), true); //TimeOut::rc_default(); //new(Duration::new(1, 0));

        let this = Self
        {

            weak_self: NonOption::invalid_rfc(), //invalid_refcell(),
            cbox,
            window_title,
            hb,
            unix_time_label,
            internal_content,
            time_out

        };

        let rc_self = Rc::new(this);

        //setup weak self reference

        rc_self_setup!(rc_self, weak_self);

        //get the state containers singletion

        let scs = StateContainers::get(); //get_state_containers();

        //add this to the GTK boxes state

        //scs.get_gtk_state_ref().get_boxes_mut().add(&rc_self);

        //scs.get_gtk_ref().get_boxes_mut().add(&rc_self);

        scs.gtk().borrow_mut_boxes().add(&rc_self);//.get_boxes_mut().add(&rc_self);

        let ws = rc_self.weak_self.borrow().get_ref().clone(); //.as_ref().clone();

        let on_timeout: Rc<SenderEventFunc<Rc<TimeOut>>> = Rc::new(move |_sender| {

            if let Some(this) = ws.upgrade()
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

        rc_self.time_out.on_time_out_subscribe(&on_timeout);

        {

            //add window contents

            //let rc_self_ref = rc_self.cbox;

            //rc_self_ref.window.set_child(Some(&rc_self_ref.contents));

            //rc_self.cbox.append(&hb);

            //rc_self_ref.window.show();

        }
        
        //contents.add_controller(controller)

        app.set_content(Some(&rc_self.cbox));

        //rc_self.time_out.set_reoccurs(true);

        rc_self.time_out.start();

        //done!

        rc_self

    }

}

impl_has_box!(cbox, WindowContentsState);

