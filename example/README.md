# Example: RTL-to-GDS with MATRIX-A5 (PicoRV32)

This is an example of using the MATRIX-A5 PDK, with PicoRV32 as the design. MATRIX-A5 ships the
*technology files*; the synthesis and place-and-route settings (constraints, floorplan, effort,
optimization) are design-specific and left to you. Below is **which tool consumes which PDK file at
each step**, described in prose so you write your own scripts. Everything keys off one entry point —
`source <library>/PDK_PATHS.tcl` — so switching libraries (e.g. `MATRIX_A5_CFET_base` →
`MATRIX_A5_CFET_bsc`) needs no script edits.

## The example design

**PicoRV32** — a small, open-source RISC-V core. We do not redistribute it; clone it directly:

```
git clone https://github.com/YosysHQ/picorv32
```

Use `picorv32.v`, top module **`picorv32`**. A template constraints file is provided here:
[`picorv32.sdc`](picorv32.sdc) — edit the clock period and I/O budget for your target.

## One entry point: `PDK_PATHS.tcl`

Sourcing it defines these variables. The names are the same in all five libraries, except
`$MMMC_FILE`, which only `CFET-base` sets:

| Variable | Points to | Used by |
|---|---|---|
| `$LIB` | `lib/…_nldm.db` (single-corner TT, compiled) | Design Compiler |
| `$LIB_TT_FILE` | `lib/…_nldm.lib` (single-corner TT, ASCII) | Genus, Innovus, PrimeTime |
| `$MMMC_FILE` | `lib/mmmc_GT_A5.tcl` (multi-corner views, `CFET-base`) | Innovus, Tempus |
| `$LEF_FILE` | `lef/` tech LEF + macro LEF + filler LEF (in order) | Innovus |
| `$PDK_MAP_FILE` | `gds/…_streamout.layermap` | Innovus streamOut |
| `$STDCELL_GDS_FILE` | `gds/…_stdcells.gds` (merged std-cell GDS) | Innovus streamOut `-merge` |
| `$QRC_TT_FILE`, `$USE_QRC` | `qrc/output.tch` | Innovus RC extraction |
| `$CTS_BUF_CELLS`, `$CTS_INV_CELLS` | clock-tree buffers / inverters | Innovus CTS |
| `$FILLER_CELLS` | spacer cells (`lef/…_fillers.lef`) | Innovus fill |
| `$SIG_BOT`, `$SIG_TOP` | frontside signal-routing range (`M0`…`M13`) | Innovus routing |
| `$PG_*`, `$CORE_PIN_LAYER`, `$SROUTE_*` | **backside** BSPDN power-grid geometry | Innovus power routing |
| `$CLK_BOT`/`$CLK_TOP`, `$CFET_SITE_*`, `$CFET_OP_*` | clock layers, site, operating point | Innovus, reporting |

The signoff files are consumed by StarRC / IC Validator directly (not through `PDK_PATHS.tcl`):
`starrc/*.itf` + `*.nxtgrd`, `models/*.inc`, `cdl/`, `drc/*.rs`, `lvs/*.rs`.

## Flow at a glance

| Stage | Tool | PDK files | You produce |
|---|---|---|---|
| Synthesis | Design Compiler **or** Genus | `$LIB` (DC) / `$LIB_TT_FILE` (Genus) | gate-level netlist + SDC |
| Place & route | Innovus | `$LEF_FILE`, `$LIB_TT_FILE`/`$MMMC_FILE`, `$QRC_TT_FILE`, `$PG_*`, `$PDK_MAP_FILE`, `$STDCELL_GDS_FILE` | routed DB + `picorv32.gds` |
| Parasitic extraction *(opt)* | StarRC | `starrc/*.nxtgrd` (from `*.itf`) + your routed DEF/LEF | SPEF |
| DRC / LVS *(opt)* | IC Validator | `drc/gtA5_cfet_main.drc.rs`, `lvs/gtA5_cfet_main.lvs.rs`, `models/*.inc`, `cdl/` | clean signoff |
| Custom layout *(opt)* | Virtuoso | `cdslib/` techfile | OA cellviews |

---

## 1. Synthesis — Design Compiler or Genus

Point the target/link library at the standard-cell Liberty: **`$LIB`** (the compiled `.db`) for
Design Compiler, or **`$LIB_TT_FILE`** (the ASCII `.lib`) for Genus / PrimeTime. Read `picorv32.v`,
elaborate the `picorv32` top, apply your constraints (`picorv32.sdc`), map, and write a gate-level
netlist + SDC. The reference flow was validated with Design Compiler; Genus reads the same `.lib`.

## 2. Place & route — Innovus

- **Libraries / corners.** Build a timing view from **`$LIB_TT_FILE`** and your mapped SDC, binding
  the QRC parasitic tech **`$QRC_TT_FILE`** into the RC corner when **`$USE_QRC`** is set.
  `CFET-base` also ships a ready multi-corner setup at **`$MMMC_FILE`** (`lib/mmmc_GT_A5.tcl`).
- **Physical init.** Initialize with the LEF list **`$LEF_FILE`** (tech + macro + filler) and your
  mapped netlist. Rows and the placement site come from the LEF.
- **Power — read this.** MATRIX-A5 uses **backside power delivery**. Route P/G on the **backside
  metals**: rings and stripes belong on `BSM1` / `BSM2` / `BSM3` (the `$PG_*` layer variables),
  tapping down to the buried power rail `BPR` (`$CORE_PIN_LAYER`), sized by the `$PG_RING_*` /
  `$PG_STRIPE_*` / `$SROUTE_*` geometry. The **frontside** metals `M0..M13`
  (`$SIG_BOT`…`$SIG_TOP`) carry **signal only**. A conventional "power on the top metals" flow is
  wrong for this PDK.
- **Clock & fill.** Clock-tree synthesis draws from the buffer / inverter lists
  **`$CTS_BUF_CELLS`** / **`$CTS_INV_CELLS`**; gaps are filled with **`$FILLER_CELLS`**.
- **RC & output.** Extract parasitics (QRC-based when `$USE_QRC`) and write a SPEF, then stream out
  with the layer map **`$PDK_MAP_FILE`**, merging the real standard-cell GDS **`$STDCELL_GDS_FILE`**
  over the LEF abstracts.

---

## Notes

- **Corners:** `CFET-base` ships a full multi-corner set via `$MMMC_FILE`
  (`lib/GT_A5_release/` — {svt, lvt, ulvt} × {TTG 25 °C, SSG/FFG at −40 °C and 125 °C}); the
  other four libraries are single-corner (`$LIB` / `$LIB_TT_FILE`).
- **Multi-Vt:** `svt` / `lvt` / `ulvt` have distinct cell names, so any combination links into one
  view. Set `LIB_VT` (one flavor, a list, or `all`) before sourcing `mmmc_GT_A5.tcl`, and pass
  `$LEF_FILE_MULTIVT` instead of `$LEF_FILE` so the `_lvt` / `_ulvt` masters are loaded. Only
  `CFET-base` ships the lvt/ulvt libraries.
- **`CFET-tall`** is a cell-level reference library only — usable for relative cell studies,
  not sign-off.
- Expected area / Fmax / power depend entirely on your constraints and effort; MATRIX-A5 does
  not prescribe them.
