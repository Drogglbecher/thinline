"""
#TL_TESTCASE(test_int_no1::check_if_sum_works)
    int test_no = 2;
    #TL_EQ[TL_FCT(no1: test_no, no2: 5) => 7]
    #TL_EQ[TL_FCT(no1: 5, no2: 2) => 7]
    EXPECT_EQ(11, test_int_no1(9, 2));
#!TL_TESTCASE
"""
def test_int_no1(no1, no2):
    return no1 + no2

class class1:
    """
    #TL_TESTCASE(test_str::check_if_str_concat_works)
        #TL_EQ[TL_FCT(str1: 'bla', str2: 'blub') => 'blablub']
    #!TL_TESTCASE
    """
    def test_str(str1, str2):
        return str1 + str2

    """
    #TL_TESTCASE(test_float::check_if_sum_works)
        #TL_EQ[TL_FCT(float1: 4.2, float2: 3.2) => 7.4]
    #!TL_TESTCASE
    """
    def test_float(float1, float2):
        return float1 + float2
