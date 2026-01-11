//! Simple and direct implementations of tests for the following event control function blocks:
//! - `E_CTU`
//! - `E_SR`
//! - `E_SWITCH`

use crate::{
    fb::{Fb, data::comm::DataBuffer},
    fb_impl::event,
    run_time,
};

pub fn test_ctu() {
    let mut rt = run_time::id_conn::IdConnRuntime::default();

    let pv = DataBuffer::UInt(100);

    let mut ctu = event::ctu::E_CTU::new("ctu0");
    ctu.write_data_in("pv", &pv);
    ctu.set_event_in("cu");

    rt.add_fb(ctu);

    for count_step in 1..=101 {
        rt.step(); // Start -> cu
        rt.step(); // cu -> Start

        {
            let fbs = rt.fbs_mut();
            let ctu_mut = fbs.get_mut("ctu0").unwrap();
            println!("[increment {count_step}]: {}", ctu_mut);
            ctu_mut.set_event_in("cu"); // receive next cu event
        }
    }
}

pub fn test_sr() {
    let mut rt = run_time::id_conn::IdConnRuntime::default();

    let mut sr = event::sr::E_SR::new("sr0");
    sr.set_event_in("s"); // Q0 -> SET
    rt.add_fb(sr);

    rt.step();
    println!("{rt}");

    {
        let fbs = rt.fbs_mut();
        let sr_mut = fbs.get_mut("sr0").unwrap();
        sr_mut.set_event_in("r");
    }

    rt.step();
    println!("{rt}");

    {
        let fbs = rt.fbs_mut();
        let sr_mut = fbs.get_mut("sr0").unwrap();
        sr_mut.set_event_in("s");
    }

    rt.step();
    println!("{rt}");
}

pub fn test_switch() {
    let mut rt = run_time::id_conn::IdConnRuntime::default();

    let t = DataBuffer::Bool(true);
    let f = DataBuffer::Bool(false);

    let mut switch = event::switch::E_SWITCH::new("switch0");

    // set switch to fire eo0
    switch.write_data_in("g", &f);
    switch.set_event_in("ei");

    let sr = event::sr::E_SR::new("sr0");

    rt.add_fb(switch);
    rt.add_fb(sr);

    rt.connect_event(("switch0", "eo0"), ("sr0", "s"));
    rt.connect_event(("switch0", "eo1"), ("sr0", "r"));

    rt.step(); // switch Start -> G0 + fires eo0
    rt.step(); // switch G0 -> Start

    println!("{rt}");

    // send event from switch0 to sr0
    println!("switch -> buffer -> sr");
    rt.send_from();
    rt.read_in();
    rt.step(); // sr: Q0 -> SET

    println!("{rt}");

    {
        let fbs_mut = rt.fbs_mut();
        let switch_mut = fbs_mut.get_mut("switch0").unwrap();

        switch_mut.write_data_in("g", &t);
        switch_mut.set_event_in("ei");
    }

    rt.step(); // switch Start -> G1 + fires eo1
    rt.step(); // switch G1 -> Start
    println!("{rt}");

    // send event from switch0 to sr0
    println!("switch -> buffer -> sr");
    rt.send_from();
    rt.read_in();
    rt.step(); // sr: Q0 -> SET

    println!("{rt}");
}
