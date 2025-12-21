pub fn run(input: &str) -> i64 {
    0
}

// First example in math:
// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
// Can be represented as:
// m * (d)
//     + n * (b + d)
//     + o * c
//     + p * (c + d)
//     + q * (a + c)
//     + r * (a + b)
// = 3a + 5b + 4c + 7d

// Rewrite first equation:
// md + nb + nd + oc + pc + pd + qa + qc + ra + rb = 3a + 5b +4c + 7d
// a(q + r) + b(n + r) + c(o + p + q) + d(m + n + p) = 3a + 5b + 4c + 7d

// Solve:
// min(S)
// where S = m + n + o + p + q + r
// m,n,o,p,q,r are none negative integers.
// and
// q + r = 3
// n + r = 5
// o + p + q = 4
// m + n + p = 7
