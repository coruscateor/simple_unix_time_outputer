
use std::cell::RefCell;

use std::rc::{Weak, Rc};

use std::time::Duration;

use gtk_estate::adw::glib::clone::Upgrade;
use gtk_estate::{gtk4 as gtk, StateContainers, StoredWidgetObject, WidgetAdapter, WidgetStateContainer};

use gtk_estate::corlib::events::SenderEventFunc;

use gtk_estate::corlib::rc_default::RcDefault;

use gtk_estate::gtk4::prelude::{BoxExt, WidgetExt};
//use gtk_estate::{HasObject, impl_has_box, impl_has_object, StateContainers}; //get_state_containers, 

use gtk_estate::gtk4::{Box, Orientation, Label, BaselinePosition, Align}; //self as gtk,

use gtk_estate::adw::{Application, ApplicationWindow, HeaderBar, WindowTitle, prelude::AdwApplicationWindowExt, gtk::prelude::ApplicationWindowExt, gtk::prelude::GtkWindowExt};

use gtk_estate::corlib::{impl_as_any, rc_self_setup, NonOption, AsAny};

use gtk_estate::time_out::*;

use time::OffsetDateTime;

use std::any::Any;

pub struct WindowContentState
{

    //weak_self: RefCell<NonOption<Weak<Self>>>,
    weak_self: Weak<Self>,
    //cbox: Box,
    window_title: WindowTitle,
    hb: HeaderBar,
    unix_time_label: Label,
    internal_content: Box,
    time_out: RcTimeOut<Weak<WindowContentState>>,
    adapted_cbox: Rc<WidgetAdapter<Box, WindowContentState>>

}

impl WindowContentState
{

    pub fn new() -> Rc<Self> //(app: &ApplicationWindow) -> Rc<Self>
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

        let unix_time_label = Label::new(Some("")); //"unix_time_label"));
        
        internal_content.append(&unix_time_label);

        internal_content.set_vexpand(true);

        internal_content.set_valign(Align::Center);

        //unix_time_label.set_valign(Align::Center);

        cbox.append(&internal_content);

        //let time_out = TimeOut::new(Duration::new(1, 0), true); //TimeOut::rc_default(); //new(Duration::new(1, 0));

        let this = Rc::new_cyclic( move |weak_self|
        {

            //let any_this: &dyn Any = weak_self;

            //let asc = any_this.downcast_ref::<Weak<dyn WidgetStateContainer>>().expect("Error: No Weak<dyn WidgetStateContainer>");

            Self
            {

                //weak_self: NonOption::invalid_rfc(), //invalid_refcell(),
                weak_self: weak_self.clone(),
                //cbox,
                window_title,
                hb,
                unix_time_label,
                internal_content,
                //time_out,
                time_out: TimeOut::with_state_ref(Duration::new(1, 0), true, weak_self),
                adapted_cbox: WidgetAdapter::new(&cbox, weak_self) //asc)

            }

        });


        //let rc_self = Rc::new(this);

        //setup weak self reference

        //rc_self_setup!(rc_self, weak_self);

        //get the state containers singletion

        let scs = StateContainers::get(); //get_state_containers();

        //add this to the GTK boxes state

        //scs.get_gtk_state_ref().get_boxes_mut().add(&rc_self);

        //scs.get_gtk_ref().get_boxes_mut().add(&rc_self);

        //scs.gtk().borrow_mut_boxes().add(&rc_self);//.get_boxes_mut().add(&rc_self);

        scs.add(&this); //&rc_self);

        //let ws = this.weak_self.clone(); //rc_self.weak_self.borrow().get_ref().clone(); //.as_ref().clone();

        let on_timeout: Rc<SenderEventFunc<RcTimeOut<Weak<WindowContentState>>>> = Rc::new(move |sender| {

            if let Some(this) = sender.state().upgrade() //ws.upgrade()
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
        
        //rc_self.time_out.on_time_out_subscribe(&on_timeout);

        this.time_out.on_time_out_subscribe(&on_timeout);

        {

            //add window contents

            //let rc_self_ref = rc_self.cbox;

            //rc_self_ref.window.set_child(Some(&rc_self_ref.contents));

            //rc_self.cbox.append(&hb);

            //rc_self_ref.window.show();

        }
        
        //contents.add_controller(controller)

        //app.set_content(Some(this.adapted_cbox.widget())); //&this.adapted_cbox.widget() //cbox)); //&rc_self.cbox));

        //rc_self.time_out.set_reoccurs(true);

        //rc_self.time_out.start();

        this.time_out.start();

        //done!

        //rc_self

        this

    }

}

impl_as_any!(WindowContentState);

impl WidgetStateContainer for WindowContentState
{

    fn dyn_adapter(&self) -> Rc<dyn StoredWidgetObject> //&(dyn gtk_estate::StoredWidgetObject)
    {

        self.adapted_cbox.clone()

    }

}



//impl_has_box!(cbox, WindowContentsState);

