macro_rules! to_i_immd_sex {
    ($e:expr) => {
        if ($e & 0x800) > 0 {
            0xffff_f000 | $e
        } else {
            $e
        }
    };
}

macro_rules! to_s_immd_zex {
    ($e:expr) => {
        ((($e >> 25) & 0x7f) << 5) | (($e >> 7) & 0x1f);
    };
}

macro_rules! to_s_immd_sex {
    ($e:expr) => {
        if ($e & 0x800) > 0 {
            0xfffff000 | $e
        } else {
            $e
        };
    };
}

macro_rules! to_b_immd_zex {
    ($e:expr) => {
        ((($e >> 31) & 1) << 12)
            | ((($e >> 7) & 1) << 11)
            | ((($e >> 25) & 0x3f) << 5)
            | (($e >> 8) & 0xf) << 1;
    };
}

macro_rules! to_b_immd_sex {
    ($e:expr) => {
        if ($e & 0x1000) > 0 {
            0xffffe000 | $e
        } else {
            $e
        };
    };
}

macro_rules! to_j_immd_zex {
    ($e:expr) => {
        ((($e >> 31) & 0x01) << 20)
            | ((($e >> 12) & 0xff) << 12)
            | ((($e >> 20) & 0x01) << 11)
            | ((($e >> 21) & 0x3ff) << 1);
    };
}

macro_rules! to_j_immd_sex {
    ($e:expr) => {
        if ($e & (1 << 20)) > 0 {
            0xffe00000 | $e
        } else {
            $e
        };
    };
}

#[cfg(test)]
mod test {
    #[test]
    fn test_to_i_immd_sex() {
        let x: u32 = to_i_immd_sex!(0x0001);
        assert_eq!(1, x);
        let x: u32 = to_i_immd_sex!(0x0800);
        assert_eq!(0xffff_f800, x);
        let x: u32 = to_i_immd_sex!(0x1800);
        assert_eq!(0xffff_f800, x);
    }
}
