pub fn find_zero_prefix_md5_hash(input: &str, check: fn(&md5::Digest) -> bool) -> usize {
    let mut counter = 0;
    loop {
        let digest = md5::compute((input.to_string() + &counter.to_string()).as_bytes());
        if check(&digest) {
            eprintln!("{input}{counter}: {:x}", digest);
            break;
        }
        counter += 1;
    }
    counter
}

pub fn part_1(input: &str) -> usize {
    find_zero_prefix_md5_hash(input, check_digest_part_1)
}

pub fn part_2(input: &str) -> usize {
    find_zero_prefix_md5_hash(input, check_digest_part_2)
}

pub fn check_digest_part_1(digest: &md5::Digest) -> bool {
    digest[0] | digest[1] | (digest[2] >> 4) == 0
}

pub fn check_digest_part_2(digest: &md5::Digest) -> bool {
    digest[0] | digest[1] | digest[2] == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(check_digest(&md5::compute("pqrstuv1048970")));
        assert!(check_digest(&md5::compute("abcdef609043")));
        assert_eq!(part_1("abcdef"), 609043);
    }
}
