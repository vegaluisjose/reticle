set part_name "xczu3eg-sbva484-1-e"

set filename [lindex $argv 0]
set dir [lindex $argv 1]
set name [lindex $argv 2]

set prefix "${dir}/${filename}"

read_verilog -sv "$prefix.v"
synth_design -mode "out_of_context" -flatten_hierarchy "rebuilt" -top $name -part $part_name
opt_design
place_design -directive Default
route_design -directive Default
write_verilog -file "${prefix}_netlist.v"
report_timing -file "${prefix}_timing.txt"
report_utilization -file "${prefix}_util.txt"
