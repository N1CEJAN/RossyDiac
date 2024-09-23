/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: Iec61499PrimitiveDatentypen
 *** Description:
 *** Version:
 *************************************************************************/

#pragma once

#include "forte_struct.h"

#include "forte_bool.h"
#include "forte_byte.h"
#include "forte_word.h"
#include "forte_dword.h"
#include "forte_lword.h"
#include "forte_sint.h"
#include "forte_int.h"
#include "forte_dint.h"
#include "forte_lint.h"
#include "forte_usint.h"
#include "forte_uint.h"
#include "forte_udint.h"
#include "forte_ulint.h"
#include "forte_real.h"
#include "forte_lreal.h"
#include "forte_char.h"
#include "forte_string.h"
#include "forte_string_fixed.h"
#include "forte_wstring.h"
#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"


class CIEC_Iec61499PrimitiveDatentypen final : public CIEC_STRUCT {
  DECLARE_FIRMWARE_DATATYPE(Iec61499PrimitiveDatentypen)

  public:
    CIEC_Iec61499PrimitiveDatentypen();

    CIEC_Iec61499PrimitiveDatentypen(const CIEC_BOOL &paa_bool, const CIEC_BYTE &paa_byte, const CIEC_WORD &paa_word, const CIEC_DWORD &paa_dword, const CIEC_LWORD &paa_lword, const CIEC_SINT &paa_sint, const CIEC_INT &paa_int, const CIEC_DINT &paa_dint, const CIEC_LINT &paa_lint, const CIEC_USINT &paa_usint, const CIEC_UINT &paa_uint, const CIEC_UDINT &paa_udint, const CIEC_ULINT &paa_ulint, const CIEC_REAL &paa_real, const CIEC_LREAL &paa_lreal, const CIEC_CHAR &paa_char, const CIEC_STRING &paa_string, const CIEC_STRING_FIXED<2> &paa_bound_string, const CIEC_WSTRING &paa_wstring, const CIEC_WSTRING &paa_bound_wstring);

    CIEC_BOOL var_a_bool;
    CIEC_BYTE var_a_byte;
    CIEC_WORD var_a_word;
    CIEC_DWORD var_a_dword;
    CIEC_LWORD var_a_lword;
    CIEC_SINT var_a_sint;
    CIEC_INT var_a_int;
    CIEC_DINT var_a_dint;
    CIEC_LINT var_a_lint;
    CIEC_USINT var_a_usint;
    CIEC_UINT var_a_uint;
    CIEC_UDINT var_a_udint;
    CIEC_ULINT var_a_ulint;
    CIEC_REAL var_a_real;
    CIEC_LREAL var_a_lreal;
    CIEC_CHAR var_a_char;
    CIEC_STRING var_a_string;
    CIEC_STRING_FIXED<2> var_a_bound_string;
    CIEC_WSTRING var_a_wstring;
    CIEC_WSTRING var_a_bound_wstring;

    size_t getStructSize() const override {
      return 20;
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


