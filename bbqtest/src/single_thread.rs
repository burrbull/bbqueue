#[cfg(test)]
mod tests {
    use bbqueue::BBQueue;

    // AJM: This test hangs/fails!
    #[test]
    fn sanity_check() {
        // Hmm, this is probably an interface smell
        let mut buf = [0u8; 6];
        let mut bb = BBQueue::new(&mut buf);

        const ITERS: usize = 100000;

        for i in 0..ITERS {
            let j = (i & 255) as u8;

            // eprintln!("===========================");
            // eprintln!("INDEX: {:?}", j);
            // eprintln!("===========================");

            // eprintln!("START: {:?}", bb);

            let wgr = bb.grant(1).unwrap();

            // eprintln!("GRANT: {:?}", bb);

            wgr.buf[0] = j;

            // eprintln!("WRITE: {:?}", bb);

            bb.commit(1, wgr);

            // eprintln!("COMIT: {:?}", bb);

            let rgr = bb.read();

            // eprintln!("READ : {:?}", bb);

            assert_eq!(rgr.buf[0], j);

            // eprintln!("RELSE: {:?}", bb);

            bb.release(1, rgr);

            // eprintln!("FINSH: {:?}", bb);
        }
    }
}
