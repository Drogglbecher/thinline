def test_int_no1(no1, no2):
    """
    #TL_TESTCASE(check_if_sum_works)
        int test_no = 2;
        #TL_EQ[TL_FCT(no1: test_no, no2: 5) => 7]
        EXPECT_EQ(11, test_int_no1(9, 2));
    #!TL_TESTCASE
    """

    return no1 + no2

class class1:
    def test_float(float1, float2):
        """
        #TL_TESTCASE(check_if_sum_works)
            #TL_EQ[TL_FCT(float1: 4.2, float2: 3.2) => 7.4]
        #!TL_TESTCASE
        """
        return float1 + float2

    def test_nodoc():
        return "No docs"

    def test_str(str1, str2):
        """
        #TL_TESTCASE(check_if_str_concat_works)
            #TL_EQ[TL_FCT(str1: 'bla', str2: 'blub') => 'blablub']
        #!TL_TESTCASE
        """

        if str.len() > 3:
            str1 + "bye"
        return str1 + str2

