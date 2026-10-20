# Minimal constraints skeleton for PicoRV32 with MATRIX-A5 -- edit for your target.
# PicoRV32 (top module `picorv32`) has one clock port `clk` and an active-low,
# asynchronous `resetn`. These are the essentials only; add uncertainty, I/O
# delays, and I/O environment appropriate to your context.

# Clock -- set your target period (ns).
create_clock -name clk -period 10.0 [get_ports clk]

# Reset is asynchronous -- keep it off the timed paths.
set_false_path -from [get_ports resetn]

# I/O timing (uncomment and budget for your context):
# set_input_delay  -clock clk <ns> [remove_from_collection [all_inputs] [get_ports clk]]
# set_output_delay -clock clk <ns> [all_outputs]
