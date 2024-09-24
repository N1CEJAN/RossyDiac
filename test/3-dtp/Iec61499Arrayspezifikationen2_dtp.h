/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: Iec61499Arrayspezifikationen2
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
#include "forte_wstring.h"
#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"


class CIEC_Iec61499Arrayspezifikationen2 final : public CIEC_STRUCT {
  DECLARE_FIRMWARE_DATATYPE(Iec61499Arrayspezifikationen2)

  public:
    CIEC_Iec61499Arrayspezifikationen2();

    CIEC_Iec61499Arrayspezifikationen2(const CIEC_ARRAY_COMMON<CIEC_BOOL> &pastatic_array_with_indexation1, const CIEC_ARRAY_COMMON<CIEC_BYTE> &pastatic_array_with_indexation2, const CIEC_ARRAY_COMMON<CIEC_WORD> &pastatic_array_with_indexation3, const CIEC_ARRAY_COMMON<CIEC_DWORD> &pastatic_array_with_indexation4, const CIEC_ARRAY_COMMON<CIEC_LWORD> &pastatic_array_with_indexation5, const CIEC_ARRAY_COMMON<CIEC_SINT> &pastatic_array_with_indexation6, const CIEC_ARRAY_COMMON<CIEC_INT> &pastatic_array_with_indexation7, const CIEC_ARRAY_COMMON<CIEC_DINT> &pastatic_array_with_indexation8, const CIEC_ARRAY_COMMON<CIEC_LINT> &pastatic_array_with_indexation9, const CIEC_ARRAY_COMMON<CIEC_USINT> &pastatic_array_with_indexation10, const CIEC_ARRAY_COMMON<CIEC_UINT> &pastatic_array_with_indexation11, const CIEC_ARRAY_COMMON<CIEC_UDINT> &pastatic_array_with_indexation12, const CIEC_ARRAY_COMMON<CIEC_ULINT> &pastatic_array_with_indexation13, const CIEC_ARRAY_COMMON<CIEC_REAL> &pastatic_array_with_indexation14, const CIEC_ARRAY_COMMON<CIEC_LREAL> &pastatic_array_with_indexation15, const CIEC_ARRAY_COMMON<CIEC_CHAR> &pastatic_array_with_indexation16, const CIEC_ARRAY_COMMON<CIEC_STRING> &pastatic_array_with_indexation17, const CIEC_ARRAY_COMMON<CIEC_WSTRING> &pastatic_array_with_indexation18);

    CIEC_ARRAY_FIXED<CIEC_BOOL, -1, 0> var_static_array_with_indexation1;
    CIEC_ARRAY_FIXED<CIEC_BYTE, -1, 0> var_static_array_with_indexation2;
    CIEC_ARRAY_FIXED<CIEC_WORD, -1, 0> var_static_array_with_indexation3;
    CIEC_ARRAY_FIXED<CIEC_DWORD, -1, 0> var_static_array_with_indexation4;
    CIEC_ARRAY_FIXED<CIEC_LWORD, -1, 0> var_static_array_with_indexation5;
    CIEC_ARRAY_FIXED<CIEC_SINT, -1, 0> var_static_array_with_indexation6;
    CIEC_ARRAY_FIXED<CIEC_INT, -1, 0> var_static_array_with_indexation7;
    CIEC_ARRAY_FIXED<CIEC_DINT, -1, 0> var_static_array_with_indexation8;
    CIEC_ARRAY_FIXED<CIEC_LINT, -1, 0> var_static_array_with_indexation9;
    CIEC_ARRAY_FIXED<CIEC_USINT, -1, 0> var_static_array_with_indexation10;
    CIEC_ARRAY_FIXED<CIEC_UINT, -1, 0> var_static_array_with_indexation11;
    CIEC_ARRAY_FIXED<CIEC_UDINT, -1, 0> var_static_array_with_indexation12;
    CIEC_ARRAY_FIXED<CIEC_ULINT, -1, 0> var_static_array_with_indexation13;
    CIEC_ARRAY_FIXED<CIEC_REAL, -1, 0> var_static_array_with_indexation14;
    CIEC_ARRAY_FIXED<CIEC_LREAL, -1, 0> var_static_array_with_indexation15;
    CIEC_ARRAY_FIXED<CIEC_CHAR, -1, 0> var_static_array_with_indexation16;
    CIEC_ARRAY_FIXED<CIEC_STRING, -1, 0> var_static_array_with_indexation17;
    CIEC_ARRAY_FIXED<CIEC_WSTRING, -1, 0> var_static_array_with_indexation18;

    size_t getStructSize() const override {
      return 18;
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


