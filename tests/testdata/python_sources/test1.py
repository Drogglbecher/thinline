"""
#TL_TESTCASE(Source1::CheckIfSumWorks)
    int test_no = 2;
    #TL_EQ[TL_FCT(no1: test_no, no2: 5) => 7]
    #TL_EQ[TL_FCT(no1: 5, no2: 2) => 7]
    EXPECT_EQ(11, test_int_no1(9, 2));
#!TL_TESTCASE
"""
def test_int_no1(no1, no2):
    return (no1 + no2);