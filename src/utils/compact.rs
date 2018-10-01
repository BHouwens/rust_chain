/**
 * The "compact" format is a representation of a whole
 * number N using an unsigned (signed?) 32bit number similar to a
 * floating point format.
 * The most significant 8 bits are the unsigned exponent of base 256.
 * This exponent can be thought of as "number of bytes of N".
 * The lower 23 bits are the mantissa.
 * Bit number 24 (0x800000) represents the sign of N.
 * N = (-1^sign) * mantissa * 256^(exponent-3)
 *
 * Satoshi's original implementation used BN_bn2mpi() and BN_mpi2bn().
 * MPI uses the most significant bit of the first byte as sign.
 * Thus 0x1234560000 is compact (0x05123456)
 * and  0xc0de000000 is compact (0x0600c0de)
 *
 * Bitcoin only uses this "compact" format for encoding difficulty
 * targets, which are unsigned 256bit quantities.  Thus, all the
 * complexities of the sign bit and using base 256 are probably an
 * implementation accident.
 */

use ramp::Int;

/**
 * Creates a "compact" format version of a Int.
 */

pub fn set_compact(bits: &u32, negative: &mut bool, overflow: &mut bool) -> Int {
    let size = bits >> 24;
    let mut word = bits & 0x007fffff;
    let mut compact_result: Int;

    if size <= 3 as u32 {
        word >>= 8 as u32 * (3 as u32 - size);
        compact_result = Int::from(word);
    } else {
        compact_result = Int::from(word);
        compact_result <<= (8 as u32 * (size - 3 as u32)) as usize;
    }

    *negative = word != 0 && (bits & 0x00800000) != 0;
    *overflow =  word != 0 && ((size > 34 as u32) ||
                 (word > 0xff && size > 33 as u32) ||
                 (word > 0xffff && size > 32 as u32));

    compact_result
}
