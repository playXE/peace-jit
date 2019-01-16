#[macro_export]
macro_rules! fast_const {
    ($base: tt : 
        $($name: ident = $e: expr),+
    ) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $name: $base = $e;
        )+
    };
}

#[macro_export]
macro_rules! is_s8 {
    ($e:expr) => {
        $e as i32 == ($e as i8 as i32)
    };
}

#[macro_export]
macro_rules! is_u8 {
    ($e:expr) => {
        $e as u32 == ($e as u8 as u32)
    };
}

#[macro_export]
macro_rules! is_s16 {
    ($e:expr) => {
        $e as i32 == ($e as i16 as i32)
    };
}

#[macro_export]
macro_rules! is_u20 {
    ($e:expr) => {
        $e & 0xfff == $e
    };
}

#[macro_export]
macro_rules! is_s20 {
    ($e:expr) => {
        (($e as i32) << 12) >> 12 == $e
    };
}

#[macro_export]
macro_rules! is_s21 {
    ($e:expr) => {
        (($e as i32) << 11) >> 11 == $e
    };
}

#[macro_export]
macro_rules! is_s24 {
    ($e:expr) => {
        (($e as i32) << 8) >> 8 == $e
    };
}

#[macro_export]
macro_rules! is_s25 {
    ($e:expr) => {
        (($e as i32) << 7) >> 7 == $e
    };
}

#[macro_export]
macro_rules! is_s32 {
    ($e:expr) => {
        $e as i64 == $e as i32 as i64
    };
}

#[macro_export]
macro_rules! is_u32 {
    ($e:expr) => {
        $e as u64 == $e as u32 as u64
    };
}