# MATRIX-A5 v1

Part of the MATRIX PDK (Memory and Advanced Transistor Research Integration Xplatform Process-Design-Kit) ecosystem

Standard-cell logic PDK for advanced 2-tier Complementary-FET (CFET) technology with backside power delivery

Developers: Minji Shon, Seongkwang Lim

PI: Prof. Shimeng Yu

Georgia Institute of Technology

## Overview

MATRIX-A5 is a complete, RTL-to-GDS-ready standard-cell PDK for Complementary-FET (CFET)
logic technologies on a shared BSIM-CMG gate-all-around (GAA) device model (the GT_A5 card,
`level=72`, with `svt / lvt / ulvt` flavors) and a backside power-delivery (BSPDN) BEOL. By
providing consistent device models and a full physical-design deck for the 2-tier CFET
technology, the platform enables reproducible design-technology co-optimization. Five CFET
process variants are included:

- **CFET 2-tier (base)** — iso-Wns complementary FET (nFET-over-pFET)
- **CFET 2-tier Wns variants** — mismatched (11/20), uniform (20/20), and 5-track high-drive (32)
- **CFET 2-tier BSC** — backside-contact power delivery

Each variant is self-contained and holds only the artifacts an RTL-to-GDS flow consumes:

- Liberty timing/power (`.lib` / `.db`) — multi-corner for `CFET-base` (SVT/LVT/ULVT ×
  TTG/SSG/FFG × temperature), single-corner for the other variants
- Technology, macro, and filler LEF, plus the merged standard-cell GDS
- Cadence QRC tech (Innovus RC) and Synopsys StarRC ITF + capacitance tables
- IC Validator DRC/LVS runsets and standard-cell CDL netlists
- The BSIM-CMG device card (GT_A5_TTG) for LVS/PEX

The compact models follow projected CFET / nanosheet scaling with backside power rails (BPR)
and backside metal (BSM1–BSM3), and are calibrated against TCAD and publicly available
industrial technology assumptions.

## Libraries

| Directory | Technology | Wns N/P (nm) | Track / row (µm) | Liberty |
|---|---|---|---|---|
| `MATRIX_A5_CFET_base`     | CFET 2-tier | 11 / 11 | 4t / 0.072 | multi-corner + single |
| `MATRIX_A5_CFET_mismatch` | CFET 2-tier | 11 / 20 | 4t / 0.072 | single-corner |
| `MATRIX_A5_CFET_uniform`  | CFET 2-tier | 20 / 20 | 4t / 0.072 | single-corner |
| `MATRIX_A5_CFET_bsc`      | CFET 2-tier, BSC PDN | 20 / 20 | 4t / 0.068 | single-corner |
| `MATRIX_A5_CFET_tall`     | CFET 2-tier, 5-track | 32 / 32 | 5t / 0.088 | single-corner (cell-level only) |

Each library directory contains `PDK_PATHS.tcl` (the Tcl manifest) plus these folders:

- `lib/` — Liberty timing/power (`.lib` + compiled `.db`)
- `lef/` — technology, macro, and filler LEF
- `gds/` — merged standard-cell GDS + Innovus streamout layermap
- `qrc/` — Cadence QRC extraction tech (`output.tch`)
- `starrc/` — Synopsys StarRC ITF + capacitance tables
- `models/` — BSIM-CMG device cards (for LVS / PEX)
- `drc/` — IC Validator DRC runset (`Include/` holds the per-layer rules)
- `lvs/` — IC Validator LVS runset + layermap
- `cdl/` — standard-cell CDL netlists
- `cdslib/` — Cadence Virtuoso / Custom Compiler technology source for custom layout
  (techfile `.tf`, display `.drf`/`.tcl`, GDS layermap)

To load `cdslib/` into Virtuoso, add `DEFINE gtA5_cfet_techfile ./gtA5_cfet_techfile` to your
`cds.lib`, then run `techLoadDump -l gtA5_cfet_techfile gtA5_cfet_techfile/gtA5_cfet_techfile.tf`
to build the OA `tech.db`.

> **The nanosheet baseline is not part of this release.** This repository ships the five
> sequential-CFET libraries only. The paper's cross-technology numbers are normalized to a
> 6-track single-tier GAA nanosheet library retargeted from the open-source **GT3** 3-nm GAAFET
> PDK (D. Shim et al., 2025 — see Acknowledgment); that retargeted baseline is not redistributed.
> GT3 itself is public and is the reference for a single-tier GAA comparison, but it targets a
> different node and BEOL stack, so it will not reproduce the NS-normalized values in the paper.

## Usage

Source a library's manifest from your synthesis (Design Compiler / Genus) and place-and-route
(Innovus) scripts:

```tcl
source /path/to/MATRIX_A5_CFET_base/PDK_PATHS.tcl
```

This defines `$LIB` / `$LIB_TT_FILE` (Liberty), `$LEF_FILE`, `$STDCELL_GDS_FILE`,
`$PDK_MAP_FILE`, `$QRC_TT_FILE` (`$USE_QRC 1`), the CTS/filler cell lists, and the BSPDN
power-grid geometry. `CFET-base` also ships a multi-corner set loaded through
`lib/mmmc_GT_A5.tcl`. Each Vt flavor has its own cell names, so `svt`, `lvt` and `ulvt` link
together: set `LIB_VT` to one flavor, a list, or `all`, and use `$LEF_FILE_MULTIVT`.

For a complete RTL-to-GDS walkthrough — which tool and which PDK file to use at each step, with
PicoRV32 as the example design — see [`example/`](example/).

To reproduce the paper's **iso-height CFlex track mixing** (loading the dense, mismatched, and wide
4-track libraries together into one synthesis and place-and-route run), see [`cflex/`](cflex/).
The three libraries share a placement site geometry and BEOL stack but ship under different SITE
names, so they need one small preparation step before they can be co-loaded.

**Note:** `MATRIX_A5_CFET_tall` is a cell-level reference library only — see its `README.md`.

**Flip-flops:**

| Cell | Clock edge | Pins |
|---|---|---|
| `*_dffpos_x1_svt`   | positive | `CLK D Q` |
| `*_dffsyn_x1_svt`   | negative | `CLK D Q` |
| `*_dffasync_x1_svt` | positive | `CLK D Q RESETN SETN` (resets active low) |

**Cell count:** **68 cells** in each of the four 4-track libraries, **67** in
`MATRIX_A5_CFET_tall`. The macro LEF carries exactly the characterized set.

**Recommended setting for `MATRIX_A5_CFET_bsc`:** mark `gtA5_cfet_bsc_20_ao32_x1_svt` as
don't-use in both synthesis and place-and-route. In the 0.068 µm BSC row it obstructs M1 pin
access of its neighbors and causes most residual routing violations. Excluding it cost no area in
our PicoRV32 benchmark.

```tcl
set_dont_use [get_lib_cells */gtA5_cfet_bsc_20_ao32_x1_svt]   ;# Design Compiler
setDontUse gtA5_cfet_bsc_20_ao32_x1_svt true                   ;# Innovus
```

## License

MATRIX-A5 is provided for academic research and educational purposes only. The framework is
publicly available for academic and non-commercial research. Copyright of the models and
infrastructure is maintained by the developers and Georgia Institute of Technology. Commercial
use, redistribution for commercial purposes, or incorporation into commercial products is
prohibited without prior written permission from Georgia Institute of Technology. See
[`LICENSE`](LICENSE) for the full terms.

Portions derived from the GT3 PDK remain under the BSD 3-Clause License — see
[`THIRD_PARTY_NOTICES`](THIRD_PARTY_NOTICES.md).

## Citation

If you use or adapt MATRIX-A5 in your work or publication, please cite the MATRIX-A5 paper:

> Minji Shon, Seongkwang Lim, Jungyoun Kwak, Jianwei Jia, Haomei Liu, and Shimeng Yu,
> "MATRIX-A5: An Open-Source CFET PDK with CFlex Track-Mixing Libraries for RTL-to-GDSII DTCO,"
> IEEE Journal on Exploratory Solid-State Computational Devices and Circuits (JXCDC), 2026.
> doi: [10.1109/JXCDC.2026.3744406](https://doi.org/10.1109/JXCDC.2026.3744406)

```bibtex
@article{shon26matrixa5,
  author  = {Minji Shon and Seongkwang Lim and Jungyoun Kwak and Jianwei Jia and Haomei Liu and Shimeng Yu},
  title   = {{MATRIX-A5}: An Open-Source {CFET} {PDK} with {CFlex} Track-Mixing Libraries for {RTL}-to-{GDSII} {DTCO}},
  journal = {IEEE J. Explor. Solid-State Comput. Devices Circuits (JXCDC)},
  year    = {2026},
  doi     = {10.1109/JXCDC.2026.3744406},
}
```

## Acknowledgment

MATRIX-A5 derives its device and power-delivery methodology from two open-source works. The
frontside GAA device stack and standard-cell flow build on the **GT3** 3-nm GAAFET PDK (D. Shim
et al., Georgia Institute of Technology, 2025), distributed under the BSD 3-Clause License. The
backside power-rail / BSPDN methodology follows the USC buried-power-rail work (Shaji et al.,
2023). We gratefully acknowledge both groups and the Laboratory for Emerging Devices and
Circuits at Georgia Tech.

```bibtex
@article{shim25gt3,
  author  = {D. Shim and others},
  title   = {{GT3}: An Open-Source 3-nm {GAAFET} {PDK} and Platform for End-to-End Evaluation of Emerging Technologies},
  journal = {IEEE Trans. Electron Devices},
  volume  = {72}, number = {4}, pages = {1582--1588}, month = apr, year = {2025},
  doi     = {10.1109/TED.2025.3540760},
}

@inproceedings{shaji23pwrrail,
  author    = {S. M. Shaji and others},
  title     = {A Comparative Study on Front-Side, Buried and Back-Side Power Rail Topologies in 3nm Technology Node},
  booktitle = {ACM/IEEE Int. Symp. Low Power Electron. Design (ISLPED)},
  month     = aug, year = {2023},
  doi       = {10.1109/ISLPED58423.2023.10244504},
}
```
