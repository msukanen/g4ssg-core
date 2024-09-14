/**
 Orbit element and star designations, in order.
 There's likely more combinations, but these will do for now.
 */
#[derive(PartialEq, Eq, Hash)]
pub enum Designation {
    // Stars & single-star orbits.
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    // Multi-star orbits.
    AB, BC, CD, DE, EF, GH, IJ, KL, MN, OP, QR, ST, UV, WX, YZ,
    ABC, BCD, CDE, DEF, EFG, FGH, GHI, HIJ, IJK, JKL, KLM, LMN, MNO, NOP, OPQ, PQR, QRS, RST, STU, TUV, UVW, VWX, WXY, XYZ,
    ABCD, BCDE, CDEF, DEFG, EFGH, FGHI, GHIJ, HIJK, IJKL, JKLM, KLMN, LMNO, MNOP, NOPQ, OPQR, PQRS, QRST, RSTU, STUV, TUVW, UVWX, VWXY, WXYZ,
    ABCDE, BCDEF, CDEFG, DEFGH, EFGHI, FGHIJ, GHIJK, HIJKL, IJKLM, JKLMN, KLMNO, LMNOP, MNOPQ, NOPQR, OPQRS, PQRST, QRSTU, RSTUV, STUVW, TUVWX, UVWXY, VWXYZ,
}
