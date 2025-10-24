use std::fmt::Write as FmtWrite;

#[allow(clippy::too_many_arguments)]
pub fn voter_fb_string(
    ecc_state: &str,
    vote: &str,
    reset: &str,
    voted: &str,
    ready: &str,
    a: &str,
    b: &str,
    c: &str,
    state: &str,
) -> String {
    let mut buf = String::new();
    let p = "";

    _ = writeln!(buf, "{p:23}{ecc_state}");
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
