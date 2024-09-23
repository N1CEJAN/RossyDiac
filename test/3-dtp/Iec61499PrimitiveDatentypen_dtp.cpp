/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: Iec61499PrimitiveDatentypen
 *** Description:
 *** Version:
 *************************************************************************/

#include "Iec61499PrimitiveDatentypen_dtp.h"
#ifdef FORTE_ENABLE_GENERATED_SOURCE_CPP
#include "Iec61499PrimitiveDatentypen_dtp_gen.cpp"
#endif

#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"

DEFINE_FIRMWARE_DATATYPE(Iec61499PrimitiveDatentypen, g_nStringIdIec61499PrimitiveDatentypen);

const CStringDictionary::TStringId CIEC_Iec61499PrimitiveDatentypen::scmElementNames[] = {g_nStringIda_bool, g_nStringIda_byte, g_nStringIda_word, g_nStringIda_dword, g_nStringIda_lword, g_nStringIda_sint, g_nStringIda_int, g_nStringIda_dint, g_nStringIda_lint, g_nStringIda_usint, g_nStringIda_uint, g_nStringIda_udint, g_nStringIda_ulint, g_nStringIda_real, g_nStringIda_lreal, g_nStringIda_char, g_nStringIda_string, g_nStringIda_bound_string, g_nStringIda_wstring, g_nStringIda_bound_wstring};

CIEC_Iec61499PrimitiveDatentypen::CIEC_Iec61499PrimitiveDatentypen() :
    CIEC_STRUCT() {
}

CIEC_Iec61499PrimitiveDatentypen::CIEC_Iec61499PrimitiveDatentypen(const CIEC_BOOL &paa_bool, const CIEC_BYTE &paa_byte, const CIEC_WORD &paa_word, const CIEC_DWORD &paa_dword, const CIEC_LWORD &paa_lword, const CIEC_SINT &paa_sint, const CIEC_INT &paa_int, const CIEC_DINT &paa_dint, const CIEC_LINT &paa_lint, const CIEC_USINT &paa_usint, const CIEC_UINT &paa_uint, const CIEC_UDINT &paa_udint, const CIEC_ULINT &paa_ulint, const CIEC_REAL &paa_real, const CIEC_LREAL &paa_lreal, const CIEC_CHAR &paa_char, const CIEC_STRING &paa_string, const CIEC_STRING_FIXED<2> &paa_bound_string, const CIEC_WSTRING &paa_wstring, const CIEC_WSTRING &paa_bound_wstring) :
    CIEC_STRUCT(),
    var_a_bool(paa_bool),
    var_a_byte(paa_byte),
    var_a_word(paa_word),
    var_a_dword(paa_dword),
    var_a_lword(paa_lword),
    var_a_sint(paa_sint),
    var_a_int(paa_int),
    var_a_dint(paa_dint),
    var_a_lint(paa_lint),
    var_a_usint(paa_usint),
    var_a_uint(paa_uint),
    var_a_udint(paa_udint),
    var_a_ulint(paa_ulint),
    var_a_real(paa_real),
    var_a_lreal(paa_lreal),
    var_a_char(paa_char),
    var_a_string(paa_string),
    var_a_bound_string(paa_bound_string),
    var_a_wstring(paa_wstring),
    var_a_bound_wstring(paa_bound_wstring) {
}

CStringDictionary::TStringId CIEC_Iec61499PrimitiveDatentypen::getStructTypeNameID() const {
  return g_nStringIdIec61499PrimitiveDatentypen;
}

void CIEC_Iec61499PrimitiveDatentypen::setValue(const CIEC_ANY &paValue) {
  if (paValue.getDataTypeID() == e_STRUCT) {
    auto &otherStruct = static_cast<const CIEC_STRUCT &>(paValue);
    if (g_nStringIdIec61499PrimitiveDatentypen == otherStruct.getStructTypeNameID()) {
      operator=(static_cast<const CIEC_Iec61499PrimitiveDatentypen &>(paValue));
    }
  }
}

CIEC_ANY *CIEC_Iec61499PrimitiveDatentypen::getMember(const size_t paIndex) {
  switch(paIndex) {
    case 0: return &var_a_bool;
    case 1: return &var_a_byte;
    case 2: return &var_a_word;
    case 3: return &var_a_dword;
    case 4: return &var_a_lword;
    case 5: return &var_a_sint;
    case 6: return &var_a_int;
    case 7: return &var_a_dint;
    case 8: return &var_a_lint;
    case 9: return &var_a_usint;
    case 10: return &var_a_uint;
    case 11: return &var_a_udint;
    case 12: return &var_a_ulint;
    case 13: return &var_a_real;
    case 14: return &var_a_lreal;
    case 15: return &var_a_char;
    case 16: return &var_a_string;
    case 17: return &var_a_bound_string;
    case 18: return &var_a_wstring;
    case 19: return &var_a_bound_wstring;
  }
  return nullptr;
}

const CIEC_ANY *CIEC_Iec61499PrimitiveDatentypen::getMember(const size_t paIndex) const {
  switch(paIndex) {
    case 0: return &var_a_bool;
    case 1: return &var_a_byte;
    case 2: return &var_a_word;
    case 3: return &var_a_dword;
    case 4: return &var_a_lword;
    case 5: return &var_a_sint;
    case 6: return &var_a_int;
    case 7: return &var_a_dint;
    case 8: return &var_a_lint;
    case 9: return &var_a_usint;
    case 10: return &var_a_uint;
    case 11: return &var_a_udint;
    case 12: return &var_a_ulint;
    case 13: return &var_a_real;
    case 14: return &var_a_lreal;
    case 15: return &var_a_char;
    case 16: return &var_a_string;
    case 17: return &var_a_bound_string;
    case 18: return &var_a_wstring;
    case 19: return &var_a_bound_wstring;
  }
  return nullptr;
}

