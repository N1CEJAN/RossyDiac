/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: ROS2_conversiontests_msg_Ros2Arrayspezifikationen1
 *** Description:
 *** Version:
 *************************************************************************/

#include "ROS2_conversiontests_msg_Ros2Arrayspezifikationen1_dtp.h"
#ifdef FORTE_ENABLE_GENERATED_SOURCE_CPP
#include "ROS2_conversiontests_msg_Ros2Arrayspezifikationen1_dtp_gen.cpp"
#endif

#include "forte_ulint.h"
#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"

DEFINE_FIRMWARE_DATATYPE(ROS2_conversiontests_msg_Ros2Arrayspezifikationen1, g_nStringIdROS2_conversiontests_msg_Ros2Arrayspezifikationen1);

const CStringDictionary::TStringId CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1::scmElementNames[] = {g_nStringIdstatic_array, g_nStringIddynamic_array, g_nStringIddynamic_array_element_counter, g_nStringIdbound_dynamic_array, g_nStringIdbound_dynamic_array_element_counter};

CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1::CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1() :
    CIEC_STRUCT(),
    var_dynamic_array_element_counter(0_ULINT),
    var_bound_dynamic_array_element_counter(0_ULINT) {
}

CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1::CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1(const CIEC_ARRAY_COMMON<CIEC_SINT> &pastatic_array, const CIEC_ARRAY_COMMON<CIEC_SINT> &padynamic_array, const CIEC_ULINT &padynamic_array_element_counter, const CIEC_ARRAY_COMMON<CIEC_SINT> &pabound_dynamic_array, const CIEC_ULINT &pabound_dynamic_array_element_counter) :
    CIEC_STRUCT(),
    var_static_array(pastatic_array),
    var_dynamic_array(padynamic_array),
    var_dynamic_array_element_counter(padynamic_array_element_counter),
    var_bound_dynamic_array(pabound_dynamic_array),
    var_bound_dynamic_array_element_counter(pabound_dynamic_array_element_counter) {
}

CStringDictionary::TStringId CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1::getStructTypeNameID() const {
  return g_nStringIdROS2_conversiontests_msg_Ros2Arrayspezifikationen1;
}

void CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1::setValue(const CIEC_ANY &paValue) {
  if (paValue.getDataTypeID() == e_STRUCT) {
    auto &otherStruct = static_cast<const CIEC_STRUCT &>(paValue);
    if (g_nStringIdROS2_conversiontests_msg_Ros2Arrayspezifikationen1 == otherStruct.getStructTypeNameID()) {
      operator=(static_cast<const CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1 &>(paValue));
    }
  }
}

CIEC_ANY *CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1::getMember(const size_t paIndex) {
  switch(paIndex) {
    case 0: return &var_static_array;
    case 1: return &var_dynamic_array;
    case 2: return &var_dynamic_array_element_counter;
    case 3: return &var_bound_dynamic_array;
    case 4: return &var_bound_dynamic_array_element_counter;
  }
  return nullptr;
}

const CIEC_ANY *CIEC_ROS2_conversiontests_msg_Ros2Arrayspezifikationen1::getMember(const size_t paIndex) const {
  switch(paIndex) {
    case 0: return &var_static_array;
    case 1: return &var_dynamic_array;
    case 2: return &var_dynamic_array_element_counter;
    case 3: return &var_bound_dynamic_array;
    case 4: return &var_bound_dynamic_array_element_counter;
  }
  return nullptr;
}

