/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: ROS2_conversiontests_msg_Ros2Referenzen
 *** Description:
 *** Version:
 *************************************************************************/

#include "ROS2_conversiontests_msg_Ros2Referenzen_dtp.h"
#ifdef FORTE_ENABLE_GENERATED_SOURCE_CPP
#include "ROS2_conversiontests_msg_Ros2Referenzen_dtp_gen.cpp"
#endif

#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"

DEFINE_FIRMWARE_DATATYPE(ROS2_conversiontests_msg_Ros2Referenzen, g_nStringIdROS2_conversiontests_msg_Ros2Referenzen);

const CStringDictionary::TStringId CIEC_ROS2_conversiontests_msg_Ros2Referenzen::scmElementNames[] = {g_nStringIdabsolute_reference, g_nStringIdrelative_reference};

CIEC_ROS2_conversiontests_msg_Ros2Referenzen::CIEC_ROS2_conversiontests_msg_Ros2Referenzen() :
    CIEC_STRUCT() {
}

CIEC_ROS2_conversiontests_msg_Ros2Referenzen::CIEC_ROS2_conversiontests_msg_Ros2Referenzen(const CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen &paabsolute_reference, const CIEC_ROS2_conversiontests_msg_Ros2PrimitiveDatentypen &parelative_reference) :
    CIEC_STRUCT(),
    var_absolute_reference(paabsolute_reference),
    var_relative_reference(parelative_reference) {
}

CStringDictionary::TStringId CIEC_ROS2_conversiontests_msg_Ros2Referenzen::getStructTypeNameID() const {
  return g_nStringIdROS2_conversiontests_msg_Ros2Referenzen;
}

void CIEC_ROS2_conversiontests_msg_Ros2Referenzen::setValue(const CIEC_ANY &paValue) {
  if (paValue.getDataTypeID() == e_STRUCT) {
    auto &otherStruct = static_cast<const CIEC_STRUCT &>(paValue);
    if (g_nStringIdROS2_conversiontests_msg_Ros2Referenzen == otherStruct.getStructTypeNameID()) {
      operator=(static_cast<const CIEC_ROS2_conversiontests_msg_Ros2Referenzen &>(paValue));
    }
  }
}

CIEC_ANY *CIEC_ROS2_conversiontests_msg_Ros2Referenzen::getMember(const size_t paIndex) {
  switch(paIndex) {
    case 0: return &var_absolute_reference;
    case 1: return &var_relative_reference;
  }
  return nullptr;
}

const CIEC_ANY *CIEC_ROS2_conversiontests_msg_Ros2Referenzen::getMember(const size_t paIndex) const {
  switch(paIndex) {
    case 0: return &var_absolute_reference;
    case 1: return &var_relative_reference;
  }
  return nullptr;
}

