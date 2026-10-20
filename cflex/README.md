# Iso-height CFlex — mixing the three 4-track libraries in one run

CFlex (CFET-flexible track mixing) composes standard cells of different nanosheet width at a
**single row height**, so synthesis and place-and-route can promote a wide, high-drive cell onto a
drive-limited path while the rest of the design stays on the dense, low-leakage cell. This
directory holds what you need to load the three 4-track libraries together.

| Library | Wns N/P (nm) | Role in the mix |
|---|---|---|
| `MATRIX_A5_CFET_base`     | 11 / 11 | dense, low leakage — the default |
| `MATRIX_A5_CFET_mismatch` | 11 / 20 | wide PMOS only (sequential CFET's independent-tier sizing) |
| `MATRIX_A5_CFET_uniform`  | 20 / 20 | wide, high drive |

## Why a preparation step is needed

The three libraries are physically compatible but ship as independent kits, so they cannot be
co-loaded straight out of the box:

- Their **site geometry is identical** — `0.039 × 0.072 µm` in all three.
- Their **tech LEFs are byte-identical apart from the SITE name** (two `diff` hunks, the `SITE`
  line and its `END`: `CoreSite` vs `CORESITE_20` vs `CORESITE_11_20`), so all 57 layers and every
  via rule already match.
- Their **QRC tech (`qrc/output.tch`) and GDS layermap are byte-identical**, so one of each serves
  the whole mix.
- Their **cell names do not collide** (`gtA5_cfet_11_*`, `gtA5_cfet_20_*`, `gtA5_cfet_11_20_*`),
  and their Liberty `library()` names are distinct, so all three link into one synthesis run.

The one obstacle is the **SITE name**. Place-and-route builds rows from a single site, and each
library's cells reference their own, so cells from the other two would not legalize. Loading all
three tech LEFs instead would redefine all 57 layers.

`make_cflex_lef.sh` resolves this by writing copies of the `_uniform` and `_mismatch` macro and
filler LEFs with their SITE renamed to `CoreSite`. **The only lines that change are `SITE` lines** —
no geometry, pin, or obstruction data is touched. The five library directories are left untouched.

## Use

```sh
sh cflex/make_cflex_lef.sh            # writes cflex/lef/ (six LEFs, ~310 KB)
```

Then, in synthesis and place-and-route, source the composite manifest instead of a per-library one:

```tcl
source /path/to/MATRIX-A5-ver1/cflex/PDK_PATHS_cflex.tcl
```

It defines the same variable names as the per-library `PDK_PATHS.tcl`, with three differences:

- `LIB`, `LIB_TT_FILE` and `STDCELL_GDS_FILE` are **lists of three** (link all three Liberty files;
  pass all three GDS to `streamOut -merge`).
- `LEF_FILE` is the `CFET-base` tech LEF plus the six site-unified LEFs from `cflex/lef/`.
- `MMMC_FILE` is **not** set — the multi-corner Liberty set ships for `CFET-base` only, so a mixed
  run is single-corner (TT, 0.7 V, 25 °C).

Everything else — backside BSPDN power routing, signal layers `M0..M13`, the QRC tech — is the same
as the per-library flow described in [`../example/`](../example/). `CTS_BUF_CELLS` and
`CTS_INV_CELLS` default to the dense `CFET-base` buffers and inverters; that is a choice, not a
measurement, and you can add the `_20` / `_11_20` drivers if you want the clock network to consider
wide cells too.

`cflex/lef/` is generated, so it is not committed — rerun the script after pulling.

## Scope

This reproduces the **iso-height** CFlex regime of the MATRIX-A5 paper, in which the tool selects
among function-equivalent cells at one row height. The paper also reports a **hybrid-row** regime
that places a taller 5-track cell (`CFET-B`, `MATRIX_A5_CFET_tall`) in alternating row bands. That
regime is **not supported by this release**: `MATRIX_A5_CFET_tall` is a cell-level reference library
only (0.088 µm row, stiff-SPF characterization — see its `README.md`), and the composite row-pattern
site it would need is not shipped here.

Absolute area, power, leakage, and Fmax depend entirely on your constraints, floorplan, and tool
effort. MATRIX-A5 ships the technology files and does not prescribe them, so expect the *direction*
of the CFlex effect to reproduce, not a specific number.
