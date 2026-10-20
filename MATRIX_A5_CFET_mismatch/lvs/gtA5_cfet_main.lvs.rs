// gtA5_cfet_main.lvs.rs -- GTA5 CFET LVS main runset (MATRIX-A5)

#define ICV_ENABLE_LVS_SEN_EXCLUDE_TOLERANCE
#include <icv.rh>

#ifdef OVERRIDE_FILE
#include "override_file.rs"
#endif


// Device properties
nmos_props : function (void) returning void
{
    vG = dev_recognition_layer();   // NGATE1
    vS = dev_pin("SRC");
    vD = dev_pin("DRN");

    vL = (dev_polygon_perim(vG)
          - dev_touch_length(vG, vS)
          - dev_touch_length(vG, vD)) * 0.5;

    vW = (dev_touch_length(vG, vS)
          + dev_touch_length(vG, vD)) * 0.5;

    dev_save_double_properties({
        {"l", vL},
        {"wgaa", vW}
    });
}

pmos_props : function (void) returning void
{
    vG = dev_recognition_layer();   // PGATE1
    vS = dev_pin("SRC");
    vD = dev_pin("DRN");

    vL = (dev_polygon_perim(vG)
          - dev_touch_length(vG, vS)
          - dev_touch_length(vG, vD)) * 0.5;

    vW = (dev_touch_length(vG, vS)
          + dev_touch_length(vG, vD)) * 0.5;

    dev_save_double_properties({
        {"l", vL},
        {"wgaa", vW}
    });
}





library(
    cell         = "cell",
    format       = GDSII,
    library_name = "inlib"
);

schematic_netlist_db = schematic(
    schematic_file = {{"sch", SPICE}}
);

vLVS_POWER_NAMES : list of string = { "VDD", "vdd", "vdd!", "VDD!" };
vLVS_GROUND_NAMES : list of string = { "VSS", "GND!", "gnd!", "vss", "vss!", "VSS!", "Vss!" };


text_options(
    allow_all_numeric             = true,
    colon_text                    = REGULAR_TEXT,
    layout_ground                 = vLVS_GROUND_NAMES,
    layout_power                  = vLVS_POWER_NAMES,
    semicolon_text                = REGULAR_TEXT
);

net_options(
    schematic_ground = vLVS_GROUND_NAMES,
    schematic_power  = vLVS_POWER_NAMES
);

////////////////////////////////////////////////////////////////////////////////////////
//
//     GTA5 CFET LVS Rules (originally adapted from GT3)
//
////////////////////////////////////////////////////////////////////////////////////////

// ============================================================================
// LAYER MAP (GTA5)
// ============================================================================
aNWELL	    = assign({ { 1 } });
aACT	    = assign({ { 2 } });
aACT2	    = assign({ { 302 } });
aGATE	    = assign({ { 3 } });
aGATE2	    = assign({ { 303 } });
aDUMMY	    = assign({ { 4 } });
aGCUT       = assign({ { 5 } });
aNSEL       = assign({ { 6 } });
aPSEL       = assign({ { 7 } });
aBPR	    = assign({ { 8 } });
tBPR	    = assign_text({ { 8 } });
aVBPR	    = assign({ { 9 } });
aVBPR2	    = assign({ { 309 } });
aSDCON      = assign({ { 10 } });
tSDCON	    = assign_text({ { 10 } });
aSDCON2     = assign({ { 310 } });
tSDCON2	    = assign_text({ { 310 } });
aVSD        = assign({ { 11 } });
aVSD2        = assign({ { 311 } });
aVSD12        = assign({ { 324 } });
aVG         = assign({ { 12 } });
aVG2         = assign({ { 312 } });
aVG12        = assign({ { 352 } });

aM0         = assign({ { 20 } });
tM0	        = assign_text({ { 20 } });
aV0         = assign({ { 22 } });
aM1         = assign({ { 25 } });
tM1	        = assign_text({ { 25 } });
aV1	        = assign({ { 27 } });
aM2	        = assign({ { 30 } });
tM2	        = assign_text({ { 30 } });
aV2         = assign({ { 32 } });
aM3         = assign({ { 35 } }); 
tM3         = assign_text({ { 35 } }); 
aV3         = assign({ { 37 } }); 
aM4         = assign({ { 40 } }); 
tM4         = assign_text({ { 40 } }); 
aV4         = assign({ { 42 } }); 
aM5         = assign({ { 45 } }); 
tM5         = assign_text({ { 45 } }); 
aV5         = assign({ { 47 } }); 
aM6         = assign({ { 50 } }); 
tM6         = assign_text({ { 50 } }); 
aV6         = assign({ { 52 } }); 
aM7         = assign({ { 55 } }); 
tM7         = assign_text({ { 55 } }); 
aV7         = assign({ { 57 } }); 
aM8         = assign({ { 60 } }); 
tM8         = assign_text({ { 60 } }); 
aV8         = assign({ { 62 } }); 
aM9         = assign({ { 65 } }); 
tM9         = assign_text({ { 65 } }); 
aV9         = assign({ { 67 } }); 
aM10        = assign({ { 70 } }); 
tM10        = assign_text({ { 70 } }); 
aV10        = assign({ { 72 } }); 
aM11        = assign({ { 75 } }); 
tM11        = assign_text({ { 75 } }); 
aV11        = assign({ { 77 } }); 
aM12        = assign({ { 80 } }); 
tM12        = assign_text({ { 80 } }); 
aV12        = assign({ { 82 } }); 
aM13        = assign({ { 85 } }); 
tM13        = assign_text({ { 85 } }); 
aV13        = assign({ { 87 } }); 
aRDL        = assign({ { 90 } }); 
tRDL        = assign_text({ { 90 } }); 

// ============================================================================
// REGION DEFINITIONS
// ============================================================================
gBULK = cell_extent( cell_list = { "*" }, exclude_text_layers = false );

// NMOS (CFET tier-2 = top device).

// convention, so NSEL is not drawn in the GDS. The tier (aACT2) alone
// discriminates polarity. If a future variant *does* draw NSEL/PSEL,
// restore `gN_NS_AREA = gN_ACTIVE_AREA and aNSEL;`.
gPOLY2 = (( ( aGATE2 ) ) not aGCUT) not aDUMMY;
gN_ACTIVE_AREA = aACT2;
gN_NS_AREA = gN_ACTIVE_AREA;
NGATE1 = gN_NS_AREA and gPOLY2;
N_src_drn = (gN_NS_AREA not NGATE1) not aDUMMY;
gN_GATE_cont = gPOLY2 not NGATE1;

// PMOS (CFET tier-1 = bottom device).

gPOLY = (( ( aGATE ) ) not aGCUT) not aDUMMY;
gP_ACTIVE_AREA = aACT;
gP_NS_AREA = gP_ACTIVE_AREA;
PGATE1 = gP_NS_AREA and gPOLY;
P_src_drn = (gP_NS_AREA not PGATE1) not aDUMMY;
gP_GATE_cont = gPOLY not PGATE1;


// ============================================================================
// CONNECTIVITY DATABASE 
// ============================================================================
CONNECT_DB : connect_database = NULL_CONNECT_DATABASE;
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY2, aM0 }, aVG2, NONE, SHIELDED_OVERLAP }} ); 
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY, aM0 }, aVG, NONE, SHIELDED_OVERLAP }} ); 
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY, gPOLY2, aM0 }, aVG, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gN_GATE_cont}, aVG12}});
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gP_GATE_cont}, aVG12}});


CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aBPR, aSDCON }, aVBPR, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aSDCON, aM0 }, aVSD, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aSDCON, aSDCON2, aM0 }, aVSD, NONE, SHIELDED_OVERLAP }} );
 
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aBPR, aSDCON2 }, aVBPR2, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aSDCON2, aM0 }, aVSD2, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aSDCON, aSDCON2 }, aVSD12, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM0, aM1 }, aV0, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM1, aM2 }, aV1, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM2, aM3 }, aV2, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM3, aM4 }, aV3, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM4, aM5 }, aV4, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM5, aM6 }, aV5, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM6, aM7 }, aV6, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM7, aM8 }, aV7, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM8, aM9 }, aV8, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM9, aM10 }, aV9, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM10, aM11 }, aV10, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM11, aM12 }, aV11, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM12, aM13 }, aV12, NONE, SHIELDED_OVERLAP }} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM13, aRDL }, aV13, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aSDCON2}, N_src_drn}} );
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aSDCON}, P_src_drn}} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gN_GATE_cont}, NGATE1}});
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gN_GATE_cont}, aVG2}});
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gN_GATE_cont}, aVG}});
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gP_GATE_cont}, PGATE1}});
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gP_GATE_cont}, aVG}});

aBPR_mark = text_origin(tBPR, cells = {"*"}, text = {"*"}, shape_size = 0.005);
aBPR_pin = and(aBPR_mark, aBPR, processing_mode = CELL_LEVEL);
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aBPR}, aBPR_pin}} );

aM0_mark = text_origin(tM0, cells = {"*"}, text = {"*"}, shape_size = 0.005);
aM0_pin = and(aM0_mark, aM0, processing_mode = CELL_LEVEL);
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM0}, aM0_pin}} );

aM1_mark = text_origin(tM1, cells = {"*"}, text = {"*"}, shape_size = 0.005);
aM1_pin = and(aM1_mark, aM1, processing_mode = CELL_LEVEL);
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM1}, aM1_pin}} );

aM2_mark = text_origin(tM2, cells = {"*"}, text = {"*"}, shape_size = 0.005);
aM2_pin = and(aM2_mark, aM2, processing_mode = CELL_LEVEL);
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM2}, aM2_pin}} );

aM3_mark = text_origin(tM3, cells = {"*"}, text = {"*"}, shape_size = 0.005);
aM3_pin = and(aM3_mark, aM3, processing_mode = CELL_LEVEL);
CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aM3}, aM3_pin}} );

CONNECT_DB = create_ports(CONNECT_DB, { {aBPR_pin, aBPR_mark}, {aM0_pin, aM0_mark}, {aM1_pin, aM1_mark}, {aM2_pin, aM2_mark}, {aM3_pin, aM3_mark}, {aM4, aM4}, {aM5, aM5}, {aM6, aM6}, {aM7, aM7}, {aM8, aM8}, {aM9, aM9}, {aM10, aM10}, {aM11, aM11}, {aM12, aM12}, {aM13, aM13}, {aRDL, aRDL} }, report = {UNUSED_MARKERS});


netlist_cdb = text_net(
    connect_sequence = CONNECT_DB,
    text_layer_items= {
    {aBPR, tBPR},
	{aM0, tM0},
	{aM1, tM1},
	{aM2, tM2},
	{aM3, tM3},
	{aM4, tM4},
	{aM5, tM5},
	{aM6, tM6},
	{aM7, tM7},
	{aM8, tM8},
	{aM9, tM9},
	{aM10, tM10},
	{aM11, tM11},
	{aM12, tM12},
	{aM13, tM13},
    {aRDL, tRDL}
    },
    attach_text= ALL,
    report_errors = {UNUSED}
);



dev_matrix = init_device_matrix(connect_sequence = netlist_cdb);

// ---- PMOS device ----
pmos(
    device_name       = "pmos_svt",
    recognition_layer = PGATE1,
    drain             = P_src_drn,
    gate              = PGATE1,
    source            = P_src_drn,
    bulk_relationship = INTERACT,
    properties        = {{"wgaa", DOUBLE, MICRO}, {"l", DOUBLE, MICRO}},
    property_function = pmos_props,
    matrix            = dev_matrix
);

// ---- NMOS device ----
nmos(
    device_name       = "nmos_svt",
    recognition_layer = NGATE1,
    drain             = N_src_drn,
    gate              = NGATE1,
    source            = N_src_drn,
    bulk_relationship = INTERACT,
    properties        = {{"wgaa", DOUBLE, MICRO}, {"l", DOUBLE, MICRO}},
    property_function = nmos_props,
    matrix            = dev_matrix
);

device_db = extract_devices(dev_matrix);
layout_netlist_db = netlist(device_db);

// ============================================================================
// LVS COMPARISON
// ============================================================================
compare_settings = init_compare_matrix();

// merge devices in parallel
merge_parallel(state = compare_settings, device_type = PMOS);
merge_parallel(state = compare_settings, device_type = NMOS);

match(
    state=compare_settings,
    match_condition={nets_matched_with_different_name = WARNING, top_layout_ports_without_name = ERROR, top_schematic_port_net_match_non_port_net = ERROR, top_layout_port_net_match_non_port_net = ERROR, top_ports_matched_with_different_name = WARNING, top_schematic_ports_matched_with_different_or_missing_name = WARNING}
);

compare(
    state=compare_settings,
    schematic = schematic_netlist_db,
    layout = layout_netlist_db
);

// ============================================================================
// PEX (NO ACTIVE LAYERS)
// ============================================================================
pex_matrix = init_pex_layer_matrix(device_db);

pex_conducting_layer_map(pex_matrix, N_src_drn, "ACT", tagname="ACTNNS");
pex_conducting_layer_map(pex_matrix, P_src_drn, "ACT", tagname="ACTPNS");
pex_conducting_layer_map(pex_matrix, gPOLY, "GATE", tagname="GATEPOLY");
pex_conducting_layer_map(pex_matrix, gPOLY2, "GATE2", tagname="GATEPOLY2");
pex_conducting_layer_map(pex_matrix, NGATE1, "GATE2", tagname="NGATE1");
pex_conducting_layer_map(pex_matrix, PGATE1, "GATE", tagname="PGATE1");
pex_conducting_layer_map(pex_matrix, gN_GATE_cont, "GATE2", tagname="gN_GATE_cont");
pex_conducting_layer_map(pex_matrix, gP_GATE_cont, "GATE", tagname="gP_GATE_cont");
pex_conducting_layer_map(pex_matrix, aSDCON, "SDCON", tagname="SDCON");
pex_conducting_layer_map(pex_matrix, aSDCON2, "SDCON2", tagname="SDCON2");
pex_conducting_layer_map(pex_matrix, aM0, "M0", tagname="M0");
pex_conducting_layer_map(pex_matrix, aM0_pin, "M0", tagname="M0_pin");
pex_conducting_layer_map(pex_matrix, aM1, "M1", tagname="M1");
pex_conducting_layer_map(pex_matrix, aM1_pin, "M1", tagname="M1_pin");
pex_conducting_layer_map(pex_matrix, aM2, "M2", tagname="M2");
pex_conducting_layer_map(pex_matrix, aM2_pin, "M2", tagname="M2_pin");
pex_conducting_layer_map(pex_matrix, aM3, "M3", tagname="M3");
pex_conducting_layer_map(pex_matrix, aM3_pin, "M3", tagname="M3_pin");
pex_conducting_layer_map(pex_matrix, aM4, "M4", tagname="M4");
pex_conducting_layer_map(pex_matrix, aM5, "M5", tagname="M5");
pex_conducting_layer_map(pex_matrix, aM6, "M6", tagname="M6");
pex_conducting_layer_map(pex_matrix, aM7, "M7", tagname="M7");
pex_conducting_layer_map(pex_matrix, aM8, "M8", tagname="M8");
pex_conducting_layer_map(pex_matrix, aM9, "M9", tagname="M9");
pex_conducting_layer_map(pex_matrix, aM10, "M10", tagname="M10");
pex_conducting_layer_map(pex_matrix, aM11, "M11", tagname="M11");
pex_conducting_layer_map(pex_matrix, aM12, "M12", tagname="M12");
pex_conducting_layer_map(pex_matrix, aM13, "M13", tagname="M13");
pex_conducting_layer_map(pex_matrix, aRDL, "RDL", tagname="RDL");
pex_conducting_layer_map(pex_matrix, aBPR, "BPR", tagname="BPR");
pex_conducting_layer_map(pex_matrix, aBPR_pin, "BPR", tagname="BPR_pin");

pex_via_layer_map(pex_matrix, aVBPR, "VBPR", tagname="VBPR");
pex_via_layer_map(pex_matrix, aVBPR2, "VBPR2", tagname="VBPR2");
pex_via_layer_map(pex_matrix, aVG, "VG", tagname="VG");
pex_via_layer_map(pex_matrix, aVG2, "VG2", tagname="VG2");
pex_via_layer_map(pex_matrix, aVG12, "VG12", tagname="VG12");
pex_via_layer_map(pex_matrix, aVSD, "VSD", tagname="VSD");
pex_via_layer_map(pex_matrix, aVSD2, "VSD2", tagname="VSD2");
pex_via_layer_map(pex_matrix, aVSD12, "VSD12", tagname="VSD12");
pex_via_layer_map(pex_matrix, aV0, "V0", tagname="V0");
pex_via_layer_map(pex_matrix, aV1, "V1", tagname="V1");
pex_via_layer_map(pex_matrix, aV2, "V2", tagname="V2");
pex_via_layer_map(pex_matrix, aV3, "V3", tagname="V3");
pex_via_layer_map(pex_matrix, aV4, "V4", tagname="V4");
pex_via_layer_map(pex_matrix, aV5, "V5", tagname="V5");
pex_via_layer_map(pex_matrix, aV6, "V6", tagname="V6");
pex_via_layer_map(pex_matrix, aV7, "V7", tagname="V7");
pex_via_layer_map(pex_matrix, aV8, "V8", tagname="V8");
pex_via_layer_map(pex_matrix, aV9, "V9", tagname="V9");
pex_via_layer_map(pex_matrix, aV10, "V10", tagname="V10");
pex_via_layer_map(pex_matrix, aV11, "V11", tagname="V11");
pex_via_layer_map(pex_matrix, aV12, "V12", tagname="V12");
pex_via_layer_map(pex_matrix, aV13, "V13", tagname="V13");


pex_layout_library_handle = gds_library("pex.gds");
pex_layout_library_layer_map_handle = pex_library_layer_map_file("pex.gds.layer.map");
pex_spice_netlist_file_handle = spice_netlist_file("pex.sp");
pex_cell_extents_file_handle = pex_cell_extents_file("pex.cell.extents");
pex_cell_port_file_handle = pex_cell_port_file("pex.cell.port");
pex_process_handle = pex_process_map_file("pex.process.map");
pex_report_handle = pex_runset_report_file("pex_runset_report");


pex_generate_database(
    pex_matrix = pex_matrix,
    pex_library = pex_layout_library_handle,
    pex_library_layer_map = pex_layout_library_layer_map_handle,
    pex_netlist_file = pex_spice_netlist_file_handle,
    pex_cell_extents_file = pex_cell_extents_file_handle,
    pex_cell_port_file = pex_cell_port_file_handle,
    pex_process_map_file = pex_process_handle,
    pex_runset_report_file = pex_report_handle
);


