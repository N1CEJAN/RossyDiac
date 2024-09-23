/*************************************************************************
 *** FORTE Library Element
 ***
 *** This file was generated using the 4DIAC FORTE Export Filter V1.0.x NG!
 ***
 *** Name: Iec61499Arrayspezifikationen1
 *** Description:
 *** Version:
 *************************************************************************/

#pragma once

#include "forte_struct.h"

#include "forte_bool.h"
#include "iec61131_functions.h"
#include "forte_array_common.h"
#include "forte_array.h"
#include "forte_array_fixed.h"
#include "forte_array_variable.h"


class CIEC_Iec61499Arrayspezifikationen1 final : public CIEC_STRUCT {
  DECLARE_FIRMWARE_DATATYPE(Iec61499Arrayspezifikationen1)

  public:
    CIEC_Iec61499Arrayspezifikationen1();

    CIEC_Iec61499Arrayspezifikationen1(const CIEC_ARRAY_COMMON<CIEC_BOOL> &pastatic_array_with_capacity, const CIEC_ARRAY_COMMON<CIEC_BOOL> &pastatic_array_with_indexation);

    CIEC_ARRAY_FIXED<CIEC_BOOL, 0, 2> var_static_array_with_capacity;
    CIEC_ARRAY_FIXED<CIEC_BOOL, -1, 1> var_static_array_with_indexation;

    size_t getStructSize() const override {
      return 2;
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


