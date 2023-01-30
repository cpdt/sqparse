use crate::combinators::Formatter;

// Helper trait for the tuple combinator.
// This trait is implemented for tuples of parsers of up to 21 elements.
pub trait Tuple<I> {
    fn format(self, input: I) -> Option<I>;
}

macro_rules! tuple_trait(
    ($name1:ident, $($name:ident),+) => (
        tuple_trait!(__impl $name1; $($name),+);
    );
    (__impl $($name:ident),+; $name1:ident, $($name2:ident),+) => (
        tuple_trait_impl!($($name),+);
        tuple_trait!(__impl $($name),+ , $name1; $($name2),+);
    );
    (__impl $($name:ident),+; $name1:ident) => (
        tuple_trait_impl!($($name),+);
        tuple_trait_impl!($($name),+ , $name1);
    );
);

macro_rules! tuple_trait_impl(
    ($($name:ident),+) => (
        impl<
            I,
            $($name : Formatter<I, I>),+
        > Tuple<I> for ( $($name),+ , ) {
            fn format(self, i: I) -> Option<I> {
                tuple_trait_inner!(0, self, i, $($name)+)
            }
        }
    );
);

macro_rules! tuple_trait_inner(
    ($index:tt, $self:expr, $input:expr, $head:ident $($id:ident)+) => ({
        let i = $self.$index.format($input)?;
        succ!($index, tuple_trait_inner!($self, i, $($id)+))
    });
    ($index:tt, $self:expr, $input:expr, $head:ident) => ({
        $self.$index.format($input)
    });
);

macro_rules! succ (
    (0, $submac:ident ! ($($rest:tt)*)) => ($submac!(1, $($rest)*));
    (1, $submac:ident ! ($($rest:tt)*)) => ($submac!(2, $($rest)*));
    (2, $submac:ident ! ($($rest:tt)*)) => ($submac!(3, $($rest)*));
    (3, $submac:ident ! ($($rest:tt)*)) => ($submac!(4, $($rest)*));
    (4, $submac:ident ! ($($rest:tt)*)) => ($submac!(5, $($rest)*));
    (5, $submac:ident ! ($($rest:tt)*)) => ($submac!(6, $($rest)*));
    (6, $submac:ident ! ($($rest:tt)*)) => ($submac!(7, $($rest)*));
    (7, $submac:ident ! ($($rest:tt)*)) => ($submac!(8, $($rest)*));
    (8, $submac:ident ! ($($rest:tt)*)) => ($submac!(9, $($rest)*));
    (9, $submac:ident ! ($($rest:tt)*)) => ($submac!(10, $($rest)*));
    (10, $submac:ident ! ($($rest:tt)*)) => ($submac!(11, $($rest)*));
    (11, $submac:ident ! ($($rest:tt)*)) => ($submac!(12, $($rest)*));
    (12, $submac:ident ! ($($rest:tt)*)) => ($submac!(13, $($rest)*));
    (13, $submac:ident ! ($($rest:tt)*)) => ($submac!(14, $($rest)*));
    (14, $submac:ident ! ($($rest:tt)*)) => ($submac!(15, $($rest)*));
    (15, $submac:ident ! ($($rest:tt)*)) => ($submac!(16, $($rest)*));
    (16, $submac:ident ! ($($rest:tt)*)) => ($submac!(17, $($rest)*));
    (17, $submac:ident ! ($($rest:tt)*)) => ($submac!(18, $($rest)*));
    (18, $submac:ident ! ($($rest:tt)*)) => ($submac!(19, $($rest)*));
    (19, $submac:ident ! ($($rest:tt)*)) => ($submac!(20, $($rest)*));
    (20, $submac:ident ! ($($rest:tt)*)) => ($submac!(21, $($rest)*));
);

tuple_trait!(F0, F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15, F16, F17, F18, F19, F20);
