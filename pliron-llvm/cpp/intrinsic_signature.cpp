// SPDX-License-Identifier: Apache-2.0
// Copyright (c) The pliron contributors

// Expose Intrinsic::getIntrinsicSignature with a C API

#include "llvm-c/Core.h"
#include "llvm/ADT/SmallVector.h"
#include "llvm/IR/DerivedTypes.h"
#include "llvm/IR/Intrinsics.h"
#include "llvm/IR/Type.h"

#include <cstddef>

extern "C" {

/// The types ID is overloaded at when instantiated as fn_ty, in the order
/// LLVMGetIntrinsicDeclaration expects them.
///
/// Writes the count to out_count and, if it fits, that many types to out_tys. Returns 0 on success,
/// -1 if fn_ty is not a function type, -2 if fn_ty is not a valid signature for ID, and -3 if OutCap
/// was too small, in which case out_tys is untouched and out_count holds the size needed.
int PlironIntrinsicGetSignature(unsigned ID, LLVMTypeRef fn_ty, LLVMTypeRef *out_tys,
                                size_t OutCap, size_t *out_count) {
  auto *f_ty = llvm::dyn_cast<llvm::FunctionType>(llvm::unwrap(fn_ty));
  if (!f_ty)
    return -1;

  llvm::SmallVector<llvm::Type *, 8> arg_tys;
  if (!llvm::Intrinsic::getIntrinsicSignature(static_cast<llvm::Intrinsic::ID>(ID), f_ty, arg_tys))
    return -2;

  *out_count = arg_tys.size();
  if (arg_tys.size() > OutCap)
    return -3;

  for (size_t i = 0; i < arg_tys.size(); ++i)
    out_tys[i] = llvm::wrap(arg_tys[i]);
  return 0;
}

} // extern "C"
