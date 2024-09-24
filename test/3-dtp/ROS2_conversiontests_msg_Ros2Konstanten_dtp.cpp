/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: ROS2_conversiontests_msg_Ros2Konstanten
 *** Description:
 *** Version:
 *************************************************************************/

#include "ROS2_conversiontests_msg_Ros2Konstanten_dtp.h"
#ifdef FORTE_ENABLE_GENERATED_SOURCE_CPP
#include "ROS2_conversiontests_msg_Ros2Konstanten_dtp_gen.cpp"
#endif

#include "forte_string.h"
#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"

DEFINE_FIRMWARE_DATATYPE(ROS2_conversiontests_msg_Ros2Konstanten, g_nStringIdROS2_conversiontests_msg_Ros2Konstanten);

const CStringDictionary::TStringId CIEC_ROS2_conversiontests_msg_Ros2Konstanten::scmElementNames[] = {g_nStringIdCONSTANT};

CIEC_ROS2_conversiontests_msg_Ros2Konstanten::CIEC_ROS2_conversiontests_msg_Ros2Konstanten() :
    CIEC_STRUCT(),
    var_CONSTANT("hallo!"_STRING) {
}

CIEC_ROS2_conversiontests_msg_Ros2Konstanten::CIEC_ROS2_conversiontests_msg_Ros2Konstanten(const CIEC_STRING &paCONSTANT) :
    CIEC_STRUCT(),
    var_CONSTANT(paCONSTANT) {
}

CStringDictionary::TStringId CIEC_ROS2_conversiontests_msg_Ros2Konstanten::getStructTypeNameID() const {
  return g_nStringIdROS2_conversiontests_msg_Ros2Konstanten;
}

void CIEC_ROS2_conversiontests_msg_Ros2Konstanten::setValue(const CIEC_ANY &paValue) {
  if (paValue.getDataTypeID() == e_STRUCT) {
    auto &otherStruct = static_cast<const CIEC_STRUCT &>(paValue);
    if (g_nStringIdROS2_conversiontests_msg_Ros2Konstanten == otherStruct.getStructTypeNameID()) {
      operator=(static_cast<const CIEC_ROS2_conversiontests_msg_Ros2Konstanten &>(paValue));
    }
  }
}

CIEC_ANY *CIEC_ROS2_conversiontests_msg_Ros2Konstanten::getMember(const size_t paIndex) {
  switch(paIndex) {
    case 0: return &var_CONSTANT;
  }
  return nullptr;
}

const CIEC_ANY *CIEC_ROS2_conversiontests_msg_Ros2Konstanten::getMember(const size_t paIndex) const {
  switch(paIndex) {
    case 0: return &var_CONSTANT;
  }
  return nullptr;
}

