#pragma once
#include "all.h"

extern Target T_amd64_sysv;
extern Target T_amd64_apple;
extern Target T_arm64;
extern Target T_arm64_apple;
extern Target T_rv64;

void codegen(FILE *, char *, FILE *, Target);
