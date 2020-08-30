OUT_DIR = $(abspath .)/out
export LIB_DIR := $(abspath .)/lib
export XILINX_VIVADO := $(XILINX_VIVADO)

NAME = dsp_add

default: test

.PHONY: test
test: xvlog xelab

.PHONY: impl
impl: | $(OUT_DIR)
	vivado -mode batch -source tcl/vivado.tcl -tclargs $(NAME) tcl/lib.tcl $(LIB_DIR) $(OUT_DIR)

xvlog:
	xvlog --sv --incr --relax -f tcl/sim.tcl

xelab:
	xelab --snapshot test --timescale 1ps/1ps -R -L unisims_ver glbl test

$(OUT_DIR):
	mkdir -p $@

clean:
	-rm -rf *.log *.wdb *.jou *.dir *.pb $(OUT_DIR) hd_visual
