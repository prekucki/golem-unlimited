extern crate rand;

use rand::prelude::*;

pub fn rand() -> u32 {
    rand::thread_rng().next_u32()
}

pub fn reqc() -> (u32, u32) {
    let x = rand() % (32 * 31);
    let i = x % 32;
    let j = x / 32;
    if j >= i {
        (i, j + 1)
    } else {
        (i, j)
    }
}

pub fn igor() -> (u32, u32) {
    let mut r = rand();
    while (r & 31) == ((r >> 5) & 31) {
        r = rand();
    }
    (r & 31, (r >> 5) & 31)
}

pub fn mbenke() -> (u32, u32) {
    let i = rand() % 32;
    let j = rand() % 31;
    (i, if j < i { j } else { j + 1 })
}

pub fn issue_author() -> (u32, u32) {
    let x = rand() % (32 * 31);
    let i = x % 32;
    let mut j = x / 32;
    if j >= i {
        j += 1;
    }
    (i, j)
}

pub fn igor_mbenke_incorrect() -> (u32, u32) {
    let r = rand();
    let i = r & 31;
    let mut j = (r >> 5) & 31;
    if j >= i {
        j += 1;
    }
    return (i, j);
}

pub fn tworec() -> (u32, u32) {
    let r = rand();
    let i = r & 31;
    let mut j = (r >> 5) & 31;

    if j != i {
        return (i, j);
    }
    j = (r >> 10) & 31;
    if j != i {
        return (i, j);
    }
    j = (r >> 15) & 31;
    if j != i {
        return (i, j);
    }
    j = (r >> 20) & 31;
    if j != i {
        return (i, j);
    }
    j = (r >> 25) & 31;
    if j != i {
        return (i, j);
    }

    if j == 31 {
        (30, 31)
    } else {
        (i, j + 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_igor_mbenke() {
        for _ in 0..100000 {
            let (a, b) = igor_mbenke_incorrect();
            assert!(a < 32);
            assert!(b < 32);
            assert_ne!(a, b);
        }
    }

    #[test]
    fn test_tworec() {
        for _ in 0..100000 {
            let (a, b) = tworec();
            assert!(a < 32);
            assert!(b < 32);
            assert_ne!(a, b);
        }
    }
}
