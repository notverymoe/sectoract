// Copyright 2023 Natalie Baker // AGPLv3 //

macro_rules! create_u16_ident {
    ($vis:vis $name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[repr(transparent)]
        $vis struct $name(u16);

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                value.0 as usize
            }
        }

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                Self(value as u16)
            }
        }
    };
}
