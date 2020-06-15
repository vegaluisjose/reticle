set top_name [lindex $argv 0]
set verilog_file [lindex $argv 1]
set utilization_file [lindex $argv 2]
set timing_file [lindex $argv 3]
set dcp_file [lindex $argv 4]

set part_name "xczu3eg-sbva484-1-e"

read_verilog -sv $verilog_file
synth_design -mode "out_of_context" -flatten_hierarchy "rebuilt" -top $top_name -part $part_name
opt_design
place_design -directive Default
route_design -directive Default
report_timing -file $timing_file
report_utilization -file $utilization_file
write_checkpoint $dcp_file
