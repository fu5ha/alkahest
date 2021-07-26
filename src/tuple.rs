use crate::{Pack, Schema, SchemaOwned, SchemaUnpack, Unpacked};

impl<'a> SchemaUnpack<'a> for () {
    type Unpacked = ();
}

impl Schema for () {
    type Packed = ();

    #[inline(always)]
    fn align() -> usize {
        1
    }

    #[inline(always)]
    fn unpack<'a>((): (), _input: &'a [u8]) {}
}

impl Pack<()> for () {
    #[inline(always)]
    fn pack(self, _offset: usize, _output: &mut [u8]) -> ((), usize) {
        ((), 0)
    }
}

impl SchemaOwned for () {
    #[inline(always)]
    fn to_owned_schema<'a>((): ()) {}
}

macro_rules! impl_for_tuple {
    ($packed_tuple:ident, [$($a:ident),+ $(,)?] [$($b:ident),+ $(,)?]) => {
        impl<'a, $($a),+> SchemaUnpack<'a> for ($($a,)+)
        where
            $($a: Schema,)+
        {
            type Unpacked = ($(Unpacked<'a, $a>,)+);
        }

        #[derive(Copy)]
        #[repr(C, packed)]
        pub struct $packed_tuple<$($a),+>($($a,)+);

        impl<$($a: Copy),+> Clone for $packed_tuple<$($a,)+> {
            #[inline(always)]
            fn clone(&self) -> Self {
                *self
            }
        }

        // `bytemuck` must be able to derive those safely. See https://github.com/Lokathor/bytemuck/issues/70
        #[allow(unsafe_code)]
        unsafe impl<$($a: bytemuck::Zeroable),+> bytemuck::Zeroable for $packed_tuple<$($a,)+> {}

        #[allow(unsafe_code)]
        unsafe impl<$($a: bytemuck::Pod),+> bytemuck::Pod for $packed_tuple<$($a,)+> {}

        impl<$($a),+> Schema for ($($a,)+)
        where
            $($a: Schema,)+
        {
            type Packed = $packed_tuple<$($a::Packed,)+>;

            #[inline(always)]
            fn align() -> usize {
                1 + ($(($a::align() - 1))|+)
            }

            #[inline(always)]
            fn unpack<'a>(packed: $packed_tuple<$($a::Packed,)+>, input: &'a [u8]) -> ($(Unpacked<'a, $a>,)+) {
                #![allow(non_snake_case)]

                let $packed_tuple($($a,)+) = packed;
                ($(<$a>::unpack($a, input),)+)
            }
        }

        impl<$($a),+ , $($b),+> Pack<($($a,)+)> for ($($b,)+)
        where
            $($a: Schema, $b: Pack<$a>,)+
        {
            #[inline]
            fn pack(self, offset: usize, output: &mut [u8]) -> ($packed_tuple<$($a::Packed,)+>, usize) {
                #![allow(non_snake_case)]

                debug_assert_eq!(
                    output.as_ptr() as usize % <($($a,)+) as Schema>::align(),
                    0,
                    "Output buffer is not aligned"
                );
                debug_assert_eq!(
                    offset % <($($a,)+) as Schema>::align(),
                    0,
                    "Offset is not aligned"
                );

                let ($($b,)+) = self;
                let mut used = 0;
                let packed = $packed_tuple( $( {
                    let aligned = (used + (<$a>::align() - 1)) & !(<$a>::align() - 1);
                    let (packed, size) = $b.pack(offset + aligned, &mut output[aligned..]);
                    used = aligned + size;
                    packed
                },)+ );
                (packed, used)
            }
        }

        impl<$($a),+> SchemaOwned for ($($a,)+)
        where
            $($a: SchemaOwned,)+
        {
            #[inline(always)]
            fn to_owned_schema<'a>(unpacked: ($(Unpacked<'a, $a>,)+)) -> ($($a,)+) {
                #![allow(non_snake_case)]

                let ($($a,)+) = unpacked;
                ($( <$a>::to_owned_schema($a) ,)+)
            }
        }
    };
}

impl_for_tuple!(PackedTuple1, [A][B]);
impl_for_tuple!(PackedTuple2, [A, B][C, D]);
impl_for_tuple!(PackedTuple3, [A, B, C][D, E, F]);
impl_for_tuple!(PackedTuple4, [A, B, C, D][E, F, G, H]);
impl_for_tuple!(PackedTuple5, [A, B, C, D, E][F, G, H, I, J]);
impl_for_tuple!(PackedTuple6, [A, B, C, D, E, F][G, H, I, J, K, L]);
impl_for_tuple!(PackedTuple7, [A, B, C, D, E, F, G][H, I, J, K, L, M, N]);
impl_for_tuple!(PackedTuple8, [A, B, C, D, E, F, G, H][I, J, K, L, M, N, O, P]);
