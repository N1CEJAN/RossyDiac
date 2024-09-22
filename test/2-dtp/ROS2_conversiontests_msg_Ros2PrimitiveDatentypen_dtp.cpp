/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: ROS2_conversiontests_msg_Ros2PrimitiveDatentypen
 *** Description:
 *** Version:
 *************************************************************************/

#include "ROS2_conversiontests_msg_Ros2PrimitiveDatentypen_dtp.h"
#ifdef FORTE_ENABLE_GENERATED_SOURCE_CPP
#include "ROS2_conversiontests_msg_Ros2PrimitiveDatentypen_dtp_gen.cpp"
#endif

#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"

DEFINE_FIRMWARE_DATATYPE(ROS2_conversiontests_msg_Ros2PrimitiveDatentypen, g_nStringIdROS2_conversiontests_msg_Ros2PrimitiveDatentypen);

const CStringDictionary::TStringId CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen::scmElementNames[] = {g_nStringIda_bool, g_nStringIda_byte, g_nStringIda_uint8, g_nStringIda_uint16, g_nStringIda_uint32, g_nStringIda_uint64, g_nStringIda_int8, g_nStringIda_int16, g_nStringIda_int32, g_nStringIda_int64, g_nStringIda_float32, g_nStringIda_float64, g_nStringIda_char, g_nStringIda_string, g_nStringIda_bound_string, g_nStringIda_wstring, g_nStringIda_bound_wstring};

CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen::CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen() :
    CIEC_STRUCT() {
}

CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen::CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen(const CIEC_BOOL &paa_bool, const CIEC_BYTE &paa_byte, const CIEC_USINT &paa_uint8, const CIEC_UINT &paa_uint16, const CIEC_UDINT &paa_uint32, const CIEC_ULINT &paa_uint64, const CIEC_SINT &paa_int8, const CIEC_INT &paa_int16, const CIEC_DINT &paa_int32, const CIEC_LINT &paa_int64, const CIEC_REAL &paa_float32, const CIEC_LREAL &paa_float64, const CIEC_CHAR &paa_char, const CIEC_STRING &paa_string, const CIEC_STRING_FIXED<2> &paa_bound_string, const CIEC_WSTRING &paa_wstring, const CIEC_WSTRING &paa_bound_wstring) :
    CIEC_STRUCT(),
    var_a_bool(paa_bool),
    var_a_byte(paa_byte),
    var_a_uint8(paa_uint8),
    var_a_uint16(paa_uint16),
    var_a_uint32(paa_uint32),
    var_a_uint64(paa_uint64),
    var_a_int8(paa_int8),
    var_a_int16(paa_int16),
    var_a_int32(paa_int32),
    var_a_int64(paa_int64),
    var_a_float32(paa_float32),
    var_a_float64(paa_float64),
    var_a_char(paa_char),
    var_a_string(paa_string),
    var_a_bound_string(paa_bound_string),
    var_a_wstring(paa_wstring),
    var_a_bound_wstring(paa_bound_wstring) {
}

CStringDictionary::TStringId CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen::getStructTypeNameID() const {
  return g_nStringIdROS2_conversiontests_msg_Ros2PrimitiveDatentypen;
}

void CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen::setValue(const CIEC_ANY &paValue) {
  if (paValue.getDataTypeID() == e_STRUCT) {
    auto &otherStruct = static_cast<const CIEC_STRUCT &>(paValue);
    if (g_nStringIdROS2_conversiontests_msg_Ros2PrimitiveDatentypen == otherStruct.getStructTypeNameID()) {
      operator=(static_cast<const CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen &>(paValue));
    }
  }
}

CIEC_ANY *CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen::getMember(const size_t paIndex) {
  switch(paIndex) {
    case 0: return &var_a_bool;
    case 1: return &var_a_byte;
    case 2: return &var_a_uint8;
    case 3: return &var_a_uint16;
    case 4: return &var_a_uint32;
    case 5: return &var_a_uint64;
    case 6: return &var_a_int8;
    case 7: return &var_a_int16;
    case 8: return &var_a_int32;
    case 9: return &var_a_int64;
    case 10: return &var_a_float32;
    case 11: return &var_a_float64;
    case 12: return &var_a_char;
    case 13: return &var_a_string;
    case 14: return &var_a_bound_string;
    case 15: return &var_a_wstring;
    case 16: return &var_a_bound_wstring;
  }
  return nullptr;
}

const CIEC_ANY *CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen::getMember(const size_t paIndex) const {
  switch(paIndex) {
    case 0: return &var_a_bool;
    case 1: return &var_a_byte;
    case 2: return &var_a_uint8;
    case 3: return &var_a_uint16;
    case 4: return &var_a_uint32;
    case 5: return &var_a_uint64;
    case 6: return &var_a_int8;
    case 7: return &var_a_int16;
    case 8: return &var_a_int32;
    case 9: return &var_a_int64;
    case 10: return &var_a_float32;
    case 11: return &var_a_float64;
    case 12: return &var_a_char;
    case 13: return &var_a_string;
    case 14: return &var_a_bound_string;
    case 15: return &var_a_wstring;
    case 16: return &var_a_bound_wstring;
  }
  return nullptr;
}

