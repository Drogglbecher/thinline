class class2:
    def test_str(str1, str2):
        if str.len() > 3:
            str1 + "bye"
        return str1 + str2

    def test_float(float1, float2):
        """
        #TL_TESTCASE(check_if_sum_works)
            #TL_EQ[TL_FCT(float1: 4.2, float2: 3.2) => 7.4]
        #!TL_TESTCASE
        """
        return float1 + float2

    def simple_text():
        """
        #TL_TESTCASE(check_output)
            #TL_EQ[TL_FCT() => 'No docs']
        #!TL_TESTCASE
        """
        return "No docs"
