# =====================================================================
# MATRIX-A5-ver1  --  MATRIX_A5_CFET_mismatch  (CFET 2-tier, mismatched Wns 11/20, 4-track)
#   Release: MATRIX-A5-ver1   Date: 2026-07-23   Card: GT_A5_TTG (BSIM-CMG level=72)
#
# Tcl manifest of the RTL-to-GDS artifacts for this PDK, sourced by
# Design Compiler / Genus (synthesis) and Innovus (P&R):
#   source <thisdir>/PDK_PATHS.tcl
# Flat foundry layout: lib/ lef/ gds/ qrc/ models/ starrc/ lvs/ cdl/ cdslib/
# NMOS Wns = 11 nm, PMOS Wns = 20 nm (mismatched-width variant).
# Single-corner Liberty only (this variant was not multi-corner characterized).
# =====================================================================

# ---- PDK root ---------------------------------------------------------------
# Resolved from this file's own location, so the kit works from wherever it is
# cloned. Override by setting PDK_ROOT yourself before sourcing this file.
if {![info exists PDK_ROOT]} {
  set PDK_ROOT [file dirname [file normalize [info script]]]
}

# ---- Liberty / DC inputs (single-corner TT, 0.7 V / 25 C) -------------------
set LIB         $PDK_ROOT/lib/gtA5_cfet_11_20_nldm.db
set LIB_TT_FILE $PDK_ROOT/lib/gtA5_cfet_11_20_nldm.lib

# ---- LEFs (tech first, then std-cell macros, then fillers) ------------------
set LEF_FILE [list \
  $PDK_ROOT/lef/gtA5_cfet.tlef \
  $PDK_ROOT/lef/gtA5_cfet_11_20_macros.lef \
  $PDK_ROOT/lef/gtA5_cfet_11_20_fillers.lef \
]

# ---- GDS streamout layermap (Innovus -> GDS) --------------------------------
set PDK_MAP_FILE $PDK_ROOT/gds/gtA5_cfet_techfile_bsm_streamout.layermap

# ---- QRC tech file (parasitic-aware RC in Innovus) --------------------------
set QRC_TT_FILE $PDK_ROOT/qrc/output.tch
set USE_QRC     1

# ---- Std-cell merged GDS for streamOut -merge -------------------------------
set STDCELL_GDS_FILE $PDK_ROOT/gds/gtA5_cfet_11_20_stdcells.gds

# ---- Power / ground rail names ----------------------------------------------
set PWR_NET    VDD
set GND_NET    VSS
set PG_PIN_PWR VDD
set PG_PIN_GND VSS
set PGNETS [list $PWR_NET $GND_NET]

# ---- Routing layer plan -----------------------------------------------------
set SIG_BOT M0
set SIG_TOP M13
set CLK_BOT M0
set CLK_TOP M6

# ---- BSPDN power grid (backside; layers must match tech LEF BSM1/2/3) --------
set PG_H BSM2
set PG_V BSM3
set PG_STRIPE_VERT BSM1
set PG_RING_WIDTH    1.0
set PG_RING_SPACING  1.1
set PG_RING_OFFSET   0.5
set PG_STRIPE_VERT_WIDTH         0.2
set PG_STRIPE_VERT_SPACING       0.8
set PG_STRIPE_VERT_SET_TO_SET    2.0
set PG_STRIPE_VERT_START_OFFSET  0.0
set PG_STRIPE_HORZ_WIDTH         0.5
set PG_STRIPE_HORZ_SPACING       1.1
set PG_STRIPE_HORZ_SET_TO_SET    3.2
set PG_STRIPE_HORZ_START_OFFSET  2.4
set CORE_PIN_LAYER       BPR
set SROUTE_LAYER_RANGE   {BPR BSM3}
set SROUTE_STRIPE_RANGE  {BSM1 BSM3}
set PROCESS_NM 45

# ---- CTS cell name lists ----------------------------------------------------
set CTS_BUF_CELLS [list \
  gtA5_cfet_11_20_buf_x1_svt gtA5_cfet_11_20_buf_x2_svt gtA5_cfet_11_20_buf_x3_svt gtA5_cfet_11_20_buf_x4_svt \
  gtA5_cfet_11_20_buf_x6_svt gtA5_cfet_11_20_buf_x8_svt gtA5_cfet_11_20_buf_x10_svt gtA5_cfet_11_20_buf_x12_svt ]
set CTS_INV_CELLS [list \
  gtA5_cfet_11_20_inv_x1_svt gtA5_cfet_11_20_inv_x2_svt gtA5_cfet_11_20_inv_x3_svt gtA5_cfet_11_20_inv_x4_svt \
  gtA5_cfet_11_20_inv_x6_svt gtA5_cfet_11_20_inv_x8_svt gtA5_cfet_11_20_inv_x10_svt gtA5_cfet_11_20_inv_x12_svt ]

# ---- Filler / spacer cells --------------------------------------------------
set FILLER_CELLS [list \
  gtA5_cfet_11_20_filler_x1_svt gtA5_cfet_11_20_filler_x2_svt gtA5_cfet_11_20_filler_x3_svt \
  gtA5_cfet_11_20_filler_x4_svt gtA5_cfet_11_20_filler_x6_svt gtA5_cfet_11_20_filler_x8_svt \
  gtA5_cfet_11_20_filler_x10_svt ]

# ---- Operating point / library metadata -------------------------------------
set CFET_OP_VOLTAGE 0.7
set CFET_OP_TEMP    25
set CFET_OP_CORNER  tt
set CFET_LIB_NAME   gtA5_cfet_11_20_nldm
set CFET_CELL_COUNT 68

# ---- Site / floorplan metadata ----------------------------------------------
set CFET_SITE_NAME CORESITE_11_20
set CFET_SITE_X    0.039
set CFET_SITE_Y    0.072

# ---- BSPDN layer / via inventory --------------------------------------------
set BSPDN_LAYERS [list BSM3 BSM2 BSM1 BPR]
set BSPDN_VIAS   [list TSV BSV1 BSV2 VBPR]

# ---- CFET 2-tier FEOL extras ------------------------------------------------
set CFET_FEOL_TIER2_LAYERS [list GATE2 ACT2 SDCON2]
set CFET_FEOL_TIER2_VIAS   [list VG2 VG12 VSD2 VSD12 VBPR2]

puts "INFO: MATRIX_A5_CFET_mismatch (MATRIX-A5-ver1) paths loaded from $PDK_ROOT/PDK_PATHS.tcl"
