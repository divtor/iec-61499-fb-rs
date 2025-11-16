use crate::{
    fb::{Fb, data::comm::DataBuffer},
    fb_impl::voter_dynamic::Voter,
    run_time,
};

pub mod interactive;

// NOTE: Failures in implementations based on this test (16-11-2025):
// - DataConnections transfer data from (v1, v2) -> v3 even though the voted event failed to sent
//      > the events failed to send since v0 sent first

// NOTE: Failures do not concern us here, we just wanted to showcase that connections work as implemented (which they do)

pub fn test_connections() {
    let mut rt = run_time::basic::Runtime::default();

    let t = DataBuffer::Bool(true);
    let f = DataBuffer::Bool(false);

    let mut v0 = Voter::new("voter0");
    v0.write_in_data("a", &t);
    v0.write_in_data("b", &t);
    v0.write_in_data("c", &f);
    v0.receive_event("vote");

    let mut v1 = Voter::new("voter1");
    v1.write_in_data("a", &t);
    v1.write_in_data("b", &f);
    v1.write_in_data("c", &t);
    v1.receive_event("vote");

    let mut v2 = Voter::new("voter2");
    v2.write_in_data("a", &f);
    v2.write_in_data("b", &t);
    v2.write_in_data("c", &t);
    v2.receive_event("vote");

    let mut v3 = Voter::new("voter3");
    v3.write_in_data("a", &f);
    v3.write_in_data("b", &f);
    v3.write_in_data("c", &f);

    rt.add_fb(v0);
    rt.add_fb(v1);
    rt.add_fb(v2);
    rt.add_fb(v3);

    rt.connect_event((0, "voted"), (3, "vote"));
    rt.connect_event((1, "voted"), (3, "vote"));
    rt.connect_event((2, "voted"), (3, "vote"));

    rt.connect_data((0, "state"), (3, "a"));
    rt.connect_data((1, "state"), (3, "b"));
    rt.connect_data((2, "state"), (3, "c"));

    println!("{rt}");

    rt.step();

    println!("{rt}");

    rt.step();

    println!("{rt}");

    rt.update_buffers();

    println!("{rt}");

    rt.read_buffers();

    println!("{rt}");

    rt.step();

    println!("{rt}");

    rt.step();

    println!("{rt}");
}
