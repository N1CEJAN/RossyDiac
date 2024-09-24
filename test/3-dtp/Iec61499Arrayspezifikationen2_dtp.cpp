/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: Iec61499Arrayspezifikationen2
 *** Description:
 *** Version:
 *************************************************************************/

#include "Iec61499Arrayspezifikationen2_dtp.h"
#ifdef FORTE_ENABLE_GENERATED_SOURCE_CPP
#include "Iec61499Arrayspezifikationen2_dtp_gen.cpp"
#endif

#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"

DEFINE_FIRMWARE_DATATYPE(Iec61499Arrayspezifikationen2, g_nStringIdIec61499Arrayspezifikationen2);

const CStringDictionary::TStringId CIEC_Iec61499Arrayspezifikationen2::scmElementNames[] = {g_nStringIdstatic_array_with_indexation1, g_nStringIdstatic_array_with_indexation2, g_nStringIdstatic_array_with_indexation3, g_nStringIdstatic_array_with_indexation4, g_nStringIdstatic_array_with_indexation5, g_nStringIdstatic_array_with_indexation6, g_nStringIdstatic_array_with_indexation7, g_nStringIdstatic_array_with_indexation8, g_nStringIdstatic_array_with_indexation9, g_nStringIdstatic_array_with_indexation10, g_nStringIdstatic_array_with_indexation11, g_nStringIdstatic_array_with_indexation12, g_nStringIdstatic_array_with_indexation13, g_nStringIdstatic_array_with_indexation14, g_nStringIdstatic_array_with_indexation15, g_nStringIdstatic_array_with_indexation16, g_nStringIdstatic_array_with_indexation17, g_nStringIdstatic_array_with_indexation18};

CIEC_Iec61499Arrayspezifikationen2::CIEC_Iec61499Arrayspezifikationen2() :
    CIEC_STRUCT() {
}

CIEC_Iec61499Arrayspezifikationen2::CIEC_Iec61499Arrayspezifikationen2(const CIEC_ARRAY_COMMON<CIEC_BOOL> &pastatic_array_with_indexation1, const CIEC_ARRAY_COMMON<CIEC_BYTE> &pastatic_array_with_indexation2, const CIEC_ARRAY_COMMON<CIEC_WORD> &pastatic_array_with_indexation3, const CIEC_ARRAY_COMMON<CIEC_DWORD> &pastatic_array_with_indexation4, const CIEC_ARRAY_COMMON<CIEC_LWORD> &pastatic_array_with_indexation5, const CIEC_ARRAY_COMMON<CIEC_SINT> &pastatic_array_with_indexation6, const CIEC_ARRAY_COMMON<CIEC_INT> &pastatic_array_with_indexation7, const CIEC_ARRAY_COMMON<CIEC_DINT> &pastatic_array_with_indexation8, const CIEC_ARRAY_COMMON<CIEC_LINT> &pastatic_array_with_indexation9, const CIEC_ARRAY_COMMON<CIEC_USINT> &pastatic_array_with_indexation10, const CIEC_ARRAY_COMMON<CIEC_UINT> &pastatic_array_with_indexation11, const CIEC_ARRAY_COMMON<CIEC_UDINT> &pastatic_array_with_indexation12, const CIEC_ARRAY_COMMON<CIEC_ULINT> &pastatic_array_with_indexation13, const CIEC_ARRAY_COMMON<CIEC_REAL> &pastatic_array_with_indexation14, const CIEC_ARRAY_COMMON<CIEC_LREAL> &pastatic_array_with_indexation15, const CIEC_ARRAY_COMMON<CIEC_CHAR> &pastatic_array_with_indexation16, const CIEC_ARRAY_COMMON<CIEC_STRING> &pastatic_array_with_indexation17, const CIEC_ARRAY_COMMON<CIEC_WSTRING> &pastatic_array_with_indexation18) :
    CIEC_STRUCT(),
    var_static_array_with_indexation1(pastatic_array_with_indexation1),
    var_static_array_with_indexation2(pastatic_array_with_indexation2),
    var_static_array_with_indexation3(pastatic_array_with_indexation3),
    var_static_array_with_indexation4(pastatic_array_with_indexation4),
    var_static_array_with_indexation5(pastatic_array_with_indexation5),
    var_static_array_with_indexation6(pastatic_array_with_indexation6),
    var_static_array_with_indexation7(pastatic_array_with_indexation7),
    var_static_array_with_indexation8(pastatic_array_with_indexation8),
    var_static_array_with_indexation9(pastatic_array_with_indexation9),
    var_static_array_with_indexation10(pastatic_array_with_indexation10),
    var_static_array_with_indexation11(pastatic_array_with_indexation11),
    var_static_array_with_indexation12(pastatic_array_with_indexation12),
    var_static_array_with_indexation13(pastatic_array_with_indexation13),
    var_static_array_with_indexation14(pastatic_array_with_indexation14),
    var_static_array_with_indexation15(pastatic_array_with_indexation15),
    var_static_array_with_indexation16(pastatic_array_with_indexation16),
    var_static_array_with_indexation17(pastatic_array_with_indexation17),
    var_static_array_with_indexation18(pastatic_array_with_indexation18) {
}

CStringDictionary::TStringId CIEC_Iec61499Arrayspezifikationen2::getStructTypeNameID() const {
  return g_nStringIdIec61499Arrayspezifikationen2;
}

void CIEC_Iec61499Arrayspezifikationen2::setValue(const CIEC_ANY &paValue) {
  if (paValue.getDataTypeID() == e_STRUCT) {
    auto &otherStruct = static_cast<const CIEC_STRUCT &>(paValue);
    if (g_nStringIdIec61499Arrayspezifikationen2 == otherStruct.getStructTypeNameID()) {
      operator=(static_cast<const CIEC_Iec61499Arrayspezifikationen2 &>(paValue));
    }
  }
}

CIEC_ANY *CIEC_Iec61499Arrayspezifikationen2::getMember(const size_t paIndex) {
  switch(paIndex) {
    case 0: return &var_static_array_with_indexation1;
    case 1: return &var_static_array_with_indexation2;
    case 2: return &var_static_array_with_indexation3;
    case 3: return &var_static_array_with_indexation4;
    case 4: return &var_static_array_with_indexation5;
    case 5: return &var_static_array_with_indexation6;
    case 6: return &var_static_array_with_indexation7;
    case 7: return &var_static_array_with_indexation8;
    case 8: return &var_static_array_with_indexation9;
    case 9: return &var_static_array_with_indexation10;
    case 10: return &var_static_array_with_indexation11;
    case 11: return &var_static_array_with_indexation12;
    case 12: return &var_static_array_with_indexation13;
    case 13: return &var_static_array_with_indexation14;
    case 14: return &var_static_array_with_indexation15;
    case 15: return &var_static_array_with_indexation16;
    case 16: return &var_static_array_with_indexation17;
    case 17: return &var_static_array_with_indexation18;
  }
  return nullptr;
}

const CIEC_ANY *CIEC_Iec61499Arrayspezifikationen2::getMember(const size_t paIndex) const {
  switch(paIndex) {
    case 0: return &var_static_array_with_indexation1;
    case 1: return &var_static_array_with_indexation2;
    case 2: return &var_static_array_with_indexation3;
    case 3: return &var_static_array_with_indexation4;
    case 4: return &var_static_array_with_indexation5;
    case 5: return &var_static_array_with_indexation6;
    case 6: return &var_static_array_with_indexation7;
    case 7: return &var_static_array_with_indexation8;
    case 8: return &var_static_array_with_indexation9;
    case 9: return &var_static_array_with_indexation10;
    case 10: return &var_static_array_with_indexation11;
    case 11: return &var_static_array_with_indexation12;
    case 12: return &var_static_array_with_indexation13;
    case 13: return &var_static_array_with_indexation14;
    case 14: return &var_static_array_with_indexation15;
    case 15: return &var_static_array_with_indexation16;
    case 16: return &var_static_array_with_indexation17;
    case 17: return &var_static_array_with_indexation18;
  }
  return nullptr;
}

