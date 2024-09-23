/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: ROS2_conversiontests_msg_Ros2Arrayspezifikationen1
 *** Description:
 *** Version:
 *************************************************************************/

#pragma once

#include "forte_struct.h"

#include "forte_sint.h"
#include "forte_ulint.h"
#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"


class CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1 final : public CIEC_STRUCT {
  DECLARE_FIRMWARE_DATATYPE(ROS2_conversiontests_msg_Ros2Arrayspezifikationen1)

  public:
    CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1();

    CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1(const CIEC_ARRAY_COMMON<CIEC_SINT> &pastatic_array, const CIEC_ARRAY_COMMON<CIEC_SINT> &padynamic_array, const CIEC_ULINT &padynamic_array_element_counter, const CIEC_ARRAY_COMMON<CIEC_SINT> &pabound_dynamic_array, const CIEC_ULINT &pabound_dynamic_array_element_counter);

    CIEC_ARRAY_FIXED<CIEC_SINT, 0, 1> var_static_array;
    CIEC_ARRAY_FIXED<CIEC_SINT, 0, 2> var_dynamic_array;
    CIEC_ULINT var_dynamic_array_element_counter;
    CIEC_ARRAY_FIXED<CIEC_SINT, 0, 1> var_bound_dynamic_array;
    CIEC_ULINT var_bound_dynamic_array_element_counter;

    size_t getStructSize() const override {
      return 5;
    }

    const CStringDictionary::TStringId* elementNames() const override {
      return scmElementNames;
    }

    CStringDictionary::TStringId getStructTypeNameID() const override;

    void setValue(const CIEC_ANY &paValue) override;

    CIEC_ANY *getMember(size_t) override;
    const CIEC_ANY *getMember(size_t) const override;

  private:
    static const CStringDictionary::TStringId scmElementNames[];

};


