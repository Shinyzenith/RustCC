#include "qbe.h"

static FILE *output_file;

Target T; // The target we want to compile to.
char debug[]; // Needed by optimization passes to emit debug information!

static void data(Dat *d) {
	emitdat(d, output_file);
	freeall();
}

static void func(Fn *fn) {
	T.abi0(fn);
	fillrpo(fn);
	fillpreds(fn);
	filluse(fn);
	promote(fn);
	filluse(fn);
	ssa(fn);
	filluse(fn);
	ssacheck(fn);
	fillalias(fn);
	loadopt(fn);
	filluse(fn);
	fillalias(fn);
	coalesce(fn);
	filluse(fn);
	ssacheck(fn);
	copy(fn);
	filluse(fn);
	fold(fn);
	T.abi1(fn);
	simpl(fn);
	fillpreds(fn);
	filluse(fn);
	T.isel(fn);
	fillrpo(fn);
	filllive(fn);
	fillloop(fn);
	fillcost(fn);
	spill(fn);
	rega(fn);
	fillrpo(fn);
	simpljmp(fn);
	fillpreds(fn);
	fillrpo(fn);
	assert(fn->rpo[0] == fn->start);
	for (uint n = 0;; n++)
		if (n == fn->nblk - 1) {
			fn->rpo[n]->link = 0;
			break;
		} else
			fn->rpo[n]->link = fn->rpo[n + 1];
	T.emitfn(fn, output_file);
	freeall();
}

void codegen(FILE *input_stream, FILE *output_stream, Target t) {
	T = t;
	output_file = output_stream;

	parse(input_stream, "", data, func);
	T.emitfin(output_stream);
}
