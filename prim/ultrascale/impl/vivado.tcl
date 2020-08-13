set top_name [lindex $argv 0]
set lib_file [lindex $argv 1]
set lib_dir [lindex $argv 2]
set out_dir [lindex $argv 3]

set utilization_file $out_dir/${top_name}_util.txt
set timing_file $out_dir/${top_name}_time.txt
set dcp_file $out_dir/${top_name}.dcp
set netlist_file $out_dir/${top_name}.v

set part_name "xczu3eg-sbva484-1-e"

source $lib_file

synth_design -mode "out_of_context" -flatten_hierarchy "rebuilt" -top $top_name -part $part_name
opt_design
place_design -directive Default
route_design -directive Default
write_verilog -file $netlist_file
report_timing -file $timing_file
report_utilization -file $utilization_file
write_checkpoint $dcp_file
