use crate::{
    fb::{Fb, data::comm::DataBuffer},
    fb_impl::voter::dynamic_disp::Voter,
    run_time,
};

pub fn test_rc_conn_par_voter() {
    let mut rt = run_time::rc_conn::RcConnRuntime::default();

    let t = DataBuffer::Bool(true);
    let f = DataBuffer::Bool(false);

    let mut v0 = Voter::new("voter0");
    v0.write_data_in("a", &t);
    v0.write_data_in("b", &t);
    v0.write_data_in("c", &f);
    v0.set_event_in("vote");

    let mut v1 = Voter::new("voter1");
    v1.write_data_in("a", &t);
    v1.write_data_in("b", &f);
    v1.write_data_in("c", &t);
    v1.set_event_in("vote");

    let mut v2 = Voter::new("voter2");
    v2.write_data_in("a", &f);
    v2.write_data_in("b", &t);
    v2.write_data_in("c", &t);
    v2.set_event_in("vote");

    let mut v3 = Voter::new("voter3");
    v3.write_data_in("a", &f);
    v3.write_data_in("b", &f);
    v3.write_data_in("c", &f);

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

    rt.send_from();

    println!("{rt}");

    rt.read_in();

    println!("{rt}");

    rt.step();

    println!("{rt}");

    rt.step();

    println!("{rt}");

    // NOTE: Failures in implementations based on this test (16-11-2025):
    // - DataConnections transfer data from (v1, v2) -> v3 even though the voted event failed to sent
    //      > the events failed to send since v0 sent first
    //      > makes sense tho?, we have buffered the data on the connection, so it can be sampled!
    // NOTE: Failures do not concern us here, we just wanted to showcase that connections work as implemented (which they do)
}

pub fn test_rc_conn_seq_voter() {
    let mut rt = run_time::rc_conn::RcConnRuntime::default();
    let t = DataBuffer::Bool(true);
    let f = DataBuffer::Bool(false);

    let mut v0 = Voter::new("voter0");
    v0.write_data_in("a", &t);
    v0.write_data_in("b", &t);
    v0.write_data_in("c", &f);
    v0.set_event_in("vote");

    let mut v1 = Voter::new("voter1");
    v1.write_data_in("a", &t);
    v1.write_data_in("b", &f);
    v1.write_data_in("c", &t);

    let mut v2 = Voter::new("voter2");
    v2.write_data_in("a", &f);
    v2.write_data_in("b", &t);
    v2.write_data_in("c", &t);

    let mut v3 = Voter::new("voter3");
    v3.write_data_in("a", &f);
    v3.write_data_in("b", &f);
    v3.write_data_in("c", &f);

    rt.add_fb(v0);
    rt.add_fb(v1);
    rt.add_fb(v2);
    rt.add_fb(v3);

    rt.connect_event((0, "voted"), (1, "vote"));
    rt.connect_event((1, "voted"), (2, "vote"));
    rt.connect_event((2, "voted"), (3, "vote"));

    rt.connect_data((0, "state"), (3, "a"));
    rt.connect_data((1, "state"), (3, "b"));
    rt.connect_data((2, "state"), (3, "c"));

    rt.connect_event((3, "voted"), (0, "reset"));
    rt.connect_event((3, "voted"), (1, "reset"));
    rt.connect_event((3, "voted"), (2, "reset"));

    println!("{rt}");

    rt.step(); // v0 ready -> vote
    rt.step(); // v0 vote -> votedpos
    rt.send_from(); // send (v0, voted) -> (v1, vote)

    println!("{rt}");

    rt.step(); // v1 ready -> vote
    rt.step(); // v1 vote -> votedpos
    rt.send_from(); // send (v1, voted) -> (v2, vote)

    println!("{rt}");

    rt.step(); // v2 ready -> vote
    rt.step(); // v2 vote -> votedpos
    rt.send_from(); // send (v2, voted) -> (v3, vote)
    rt.read_in(); // fetch (v0, v1, v2) state buffers into v3

    println!("{rt}");

    rt.step(); // v3 ready -> vote
    rt.step(); // v3 vote -> votedpos

    println!("{rt}");

    rt.send_from(); // send (v3, voted) -> (v0-2, reset)

    println!("{rt}");

    rt.step(); // v0-2 votedpos -> reset
    rt.step(); // v0-2 reset -> ready

    println!("{rt}");
}

pub fn test_id_conn_seq_voter() {
    let mut rt = run_time::id_conn::IdConnRuntime::default();
    let t = DataBuffer::Bool(true);
    let f = DataBuffer::Bool(false);

    let mut v0 = Voter::new("voter0");
    v0.write_data_in("a", &t);
    v0.write_data_in("b", &t);
    v0.write_data_in("c", &f);
    v0.set_event_in("vote");

    let mut v1 = Voter::new("voter1");
    v1.write_data_in("a", &t);
    v1.write_data_in("b", &f);
    v1.write_data_in("c", &t);

    let mut v2 = Voter::new("voter2");
    v2.write_data_in("a", &f);
    v2.write_data_in("b", &t);
    v2.write_data_in("c", &t);

    let mut v3 = Voter::new("voter3");
    v3.write_data_in("a", &f);
    v3.write_data_in("b", &f);
    v3.write_data_in("c", &f);

    rt.add_fb(v0);
    rt.add_fb(v1);
    rt.add_fb(v2);
    rt.add_fb(v3);

    rt.connect_event(("voter0", "voted"), ("voter1", "vote"));
    rt.connect_event(("voter1", "voted"), ("voter2", "vote"));
    rt.connect_event(("voter2", "voted"), ("voter3", "vote"));

    rt.connect_data(("voter0", "state"), ("voter3", "a"));
    rt.connect_data(("voter1", "state"), ("voter3", "b"));
    rt.connect_data(("voter2", "state"), ("voter3", "c"));

    rt.connect_event(("voter3", "voted"), ("voter0", "reset"));
    rt.connect_event(("voter3", "voted"), ("voter1", "reset"));
    rt.connect_event(("voter3", "voted"), ("voter2", "reset"));

    println!("{rt}");

    rt.step(); // v0 ready -> vote
    rt.step(); // v0 vote -> votedpos
    rt.send_from(); // send (v0, voted) -> (v1, vote)

    println!("{rt}");

    rt.step(); // v1 ready -> vote
    rt.step(); // v1 vote -> votedpos
    rt.send_from(); // send (v1, voted) -> (v2, vote)

    println!("{rt}");

    rt.step(); // v2 ready -> vote
    rt.step(); // v2 vote -> votedpos
    rt.send_from(); // send (v2, voted) -> (v3, vote)
    rt.read_in(); // fetch (v0, v1, v2) state buffers into v3

    println!("{rt}");

    rt.step(); // v3 ready -> vote
    rt.step(); // v3 vote -> votedpos

    println!("{rt}");

    rt.send_from(); // send (v3, voted) -> (v0-2, reset)

    println!("{rt}");

    rt.step(); // v0-2 votedpos -> reset
    rt.step(); // v0-2 reset -> ready

    println!("{rt}");
}
