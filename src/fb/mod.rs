use std::{cell::RefCell, rc::Rc};

use data::comm::CommunicationData;

pub mod data;
pub mod direction;
pub mod event;

pub trait Fb {
    fn instance_name(&self) -> &'static str;
    fn receive_event(&mut self, event: &str);
    fn send_event(&mut self, event: &str);
    fn active_in_event(&self) -> Option<&'static str>;
    fn active_out_event(&self) -> Option<&'static str>;
    fn event_associations(&self, event: &str) -> Vec<&'static str>;
    fn read_output(&self, data: &str) -> CommunicationData;
    fn write_input(&mut self, data: &str, value: &CommunicationData);
    fn invoke_ecc(&mut self) -> bool;
}

pub type FbRef = Rc<RefCell<dyn Fb>>;
