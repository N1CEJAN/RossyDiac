/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: Iec61499Arrayspezifikationen1
 *** Description:
 *** Version:
 *************************************************************************/

#include "Iec61499Arrayspezifikationen1_dtp.h"
#ifdef FORTE_ENABLE_GENERATED_SOURCE_CPP
#include "Iec61499Arrayspezifikationen1_dtp_gen.cpp"
#endif

#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"

DEFINE_FIRMWARE_DATATYPE(Iec61499Arrayspezifikationen1, g_nStringIdIec61499Arrayspezifikationen1);

const CStringDictionary::TStringId CIEC_Iec61499Arrayspezifikationen1::scmElementNames[] = {g_nStringIdstatic_array_with_capacity, g_nStringIdstatic_array_with_indexation};

CIEC_Iec61499Arrayspezifikationen1::CIEC_Iec61499Arrayspezifikationen1() :
    CIEC_STRUCT() {
}

CIEC_Iec61499Arrayspezifikationen1::CIEC_Iec61499Arrayspezifikationen1(const CIEC_ARRAY_COMMON<CIEC_BOOL> &pastatic_array_with_capacity, const CIEC_ARRAY_COMMON<CIEC_BOOL> &pastatic_array_with_indexation) :
    CIEC_STRUCT(),
    var_static_array_with_capacity(pastatic_array_with_capacity),
    var_static_array_with_indexation(pastatic_array_with_indexation) {
}

CStringDictionary::TStringId CIEC_Iec61499Arrayspezifikationen1::getStructTypeNameID() const {
  return g_nStringIdIec61499Arrayspezifikationen1;
}

void CIEC_Iec61499Arrayspezifikationen1::setValue(const CIEC_ANY &paValue) {
  if (paValue.getDataTypeID() == e_STRUCT) {
    auto &otherStruct = static_cast<const CIEC_STRUCT &>(paValue);
    if (g_nStringIdIec61499Arrayspezifikationen1 == otherStruct.getStructTypeNameID()) {
      operator=(static_cast<const CIEC_Iec61499Arrayspezifikationen1 &>(paValue));
    }
  }
}

CIEC_ANY *CIEC_Iec61499Arrayspezifikationen1::getMember(const size_t paIndex) {
  switch(paIndex) {
    case 0: return &var_static_array_with_capacity;
    case 1: return &var_static_array_with_indexation;
  }
  return nullptr;
}

const CIEC_ANY *CIEC_Iec61499Arrayspezifikationen1::getMember(const size_t paIndex) const {
  switch(paIndex) {
    case 0: return &var_static_array_with_capacity;
    case 1: return &var_static_array_with_indexation;
  }
  return nullptr;
}

