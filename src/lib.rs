pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_android() {
        use procfs::process::Process;
        let me = Process::myself();
        assert!(me.is_ok());
        let me = me.unwrap();
        let fd_count = me.fd_count();
        assert!(fd_count.is_ok());
        let fd_count = fd_count.unwrap();
        let fd_count: u64 = fd_count.try_into().unwrap();
        for f in me.fd().unwrap() {
            let fd = f.unwrap();
            println!("{:?} {:?}", fd, fd.mode());
        }
    }
}
