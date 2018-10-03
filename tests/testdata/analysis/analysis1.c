/**
 * # TESTCASE(Source1::CheckIfSumWorks)
 *     int test_no = 2;
 *     # EQ[TL_FCT(no1: test_no, no2: 5) => 7]
 *     # EQ[TL_FCT(no1: 5, no2: 2) => 7]
 *     EXPECT_EQ(11, test_int_no1(9, 2));
 */
int test_int_no1(int no1, int no2) {
    return (no1 + no2);
}

/**
 * # TESTCASE(Source1::TestPtr)
 *     int test_no = 2;
 *     int test_no2 = 5;
 *     # EQ[TL_FCT(no1: &test_no, no2: &test_no2) => 7]
 */
int test_ptr(const int * const no1, const int * const no2) {
    return (*no1 + *no2);
}

/**
 * # TESTCASE(Source1::EmptyFct)
 *     # EQ[TL_FCT() => 7]
 *     # NE[TL_FCT() => 4]
 */
int test_empty_fct() {
    return 7;
}

/**
 * This function has parameters, yeah
 */
int main(const int argc, char * const argv[]) {
    test_int_no1(1, 2);
}
