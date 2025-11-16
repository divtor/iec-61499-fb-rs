use std::fmt::Write as FmtWrite;

pub struct VoterInformation {
    pub ecc: &'static str,
    pub vote: &'static str,
    pub reset: &'static str,
    pub voted: &'static str,
    pub ready: &'static str,
    pub a: &'static str,
    pub b: &'static str,
    pub c: &'static str,
    pub state: &'static str,
}

pub fn voter_str(info: VoterInformation) -> String {
    let ecc = info.ecc;
    let vote = info.vote;
    let reset = info.reset;
    let voted = info.voted;
    let ready = info.ready;
    let a = info.a;
    let b = info.b;
    let c = info.c;
    let state = info.state;

    let mut buf = String::new();
    let p = "";

    _ = writeln!(buf, "{p:23}{ecc}");
    _ = writeln!(buf, "{p:10}     +----------------------+");
    _ = writeln!(buf, "{vote:10}--x--|Vote             Voted|-x--- {voted}");
    _ = writeln!(buf, "{p:10}  |  |                      | |");
    _ = writeln!(buf, "{reset:10}-----|Reset            Ready|---x- {ready}");
    _ = writeln!(buf, "{p:10}  |  |                      | | |");
    _ = writeln!(buf, "{p:10}  |  +-+                  +-+ | |");
    _ = writeln!(buf, "{p:10}  |    |      VOTER       |   | |");
    _ = writeln!(buf, "{p:10}  |  +-+                  +-+ | |");
    _ = writeln!(buf, "{p:10}  |  |                      | | |");
    _ = writeln!(buf, "{a:10}--x--|A                State|-x-x- {state}");
    _ = writeln!(buf, "{p:10}  |  |                      |");
    _ = writeln!(buf, "{b:10}--x--|B                     |");
    _ = writeln!(buf, "{p:10}  |  |                      |");
    _ = writeln!(buf, "{c:10}--x--|C                     |");
    _ = writeln!(buf, "{p:10}     +----------------------+");

    buf
}

pub fn voter_str_w_name(info: VoterInformation, instance_name: &str) -> String {
    let ecc = info.ecc;
    let vote = info.vote;
    let reset = info.reset;
    let voted = info.voted;
    let ready = info.ready;
    let a = info.a;
    let b = info.b;
    let c = info.c;
    let state = info.state;

    let mut buf = String::new();
    let p = "";

    _ = writeln!(buf, "{p:18}{instance_name}     {ecc}");
    _ = writeln!(buf, "{p:10}     +----------------------+");
    _ = writeln!(buf, "{vote:10}--x--|Vote             Voted|-x--- {voted}");
    _ = writeln!(buf, "{p:10}  |  |                      | |");
    _ = writeln!(buf, "{reset:10}-----|Reset            Ready|---x- {ready}");
    _ = writeln!(buf, "{p:10}  |  |                      | | |");
    _ = writeln!(buf, "{p:10}  |  +-+                  +-+ | |");
    _ = writeln!(buf, "{p:10}  |    |      VOTER       |   | |");
    _ = writeln!(buf, "{p:10}  |  +-+                  +-+ | |");
    _ = writeln!(buf, "{p:10}  |  |                      | | |");
    _ = writeln!(buf, "{a:10}--x--|A                State|-x-x- {state}");
    _ = writeln!(buf, "{p:10}  |  |                      |");
    _ = writeln!(buf, "{b:10}--x--|B                     |");
    _ = writeln!(buf, "{p:10}  |  |                      |");
    _ = writeln!(buf, "{c:10}--x--|C                     |");
    _ = writeln!(buf, "{p:10}     +----------------------+");

    buf
}
