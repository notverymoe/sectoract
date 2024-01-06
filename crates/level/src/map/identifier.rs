// Copyright 2023 Natalie Baker // AGPLv3 //

macro_rules! create_ident {
    ($inner:ty, $vis:vis $name:ident) => {
        #[derive(Debug, Clone, Copy, bytemuck::Zeroable, bytemuck::Pod, bytemuck::ByteHash, bytemuck::ByteEq)]
        #[repr(transparent)]
        $vis struct $name($inner);

        impl From<$name> for usize {
            fn from(value: $name) -> Self {
                value.0 as usize
            }
        }

        impl From<usize> for $name {
            fn from(value: usize) -> Self {
                Self(value as $inner)
            }
        }

        impl $name {
            #[must_use]
            pub const fn to_raw(self) -> $inner {
                self.0
            }

            #[must_use]
            pub const fn from_raw(value: $inner) -> Self {
                Self(value)
            }

            #[must_use]
            pub const fn const_eq(self, other: Self) -> bool {
                constmuck::cast::<Self, $inner>(self) == constmuck::cast::<Self, $inner>(other)
            }
        }
    };
}
