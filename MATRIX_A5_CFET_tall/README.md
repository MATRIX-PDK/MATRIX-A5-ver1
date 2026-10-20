# MATRIX_A5_CFET_tall — CELL-LEVEL REFERENCE ONLY

5-track, 0.088 µm row, symmetric **Wns = 32 nm** CFET 2-tier variant. It exists to study
the wide-Wns high-drive device at the **cell level**.

## Read this before using the Liberty

**It is characterized with a stiff-SPF `cmin=1e-16` + `gear2` recipe.** The Wns-32 device
has very low ON-resistance, so the extracted post-layout nets are numerically stiff; the
recipe is the only way they converge. It **inflates input pin capacitance ~3.5×** (roughly
uniform across cells). Timing/power/CTS-buffer counts from this lib are therefore **upper
bounds, not nominal values**.

## What this library IS good for

Relative, characterization-robust cell-level ratios: **drive (~1.6× vs 4t-20 by Rout slope,
a device I-V property), leakage (~1.6×, device), cell area (~1.22×, LEF geometry), input pin
cap (~1.3× ≈ 0.33 fF, width-scaled)**. Cite these as ratios.
