// gtA5_cfet_main.drc.rs -- GTA5 CFET DRC main runset (MATRIX-A5)

#define SELECTABLE_VIOLATION_NAMES
#define ICV_ENABLE_WIDE_ANGLED
#include <icv.rh>
#include "string.rh"

// Layer name mapping to IDs
aBPR	    = assign({ { 8 } });
tBPR	    = assign_text({ { 8 } });
aVBPR	    = assign({ { 9 } });
aVBPR2	    = assign({ { 309 } });
aNW	    = assign({ { 1 } });
aACT	    = assign({ { 2 } });
aACT2	    = assign({ { 302 } });
aGATE	    = assign({ { 3 } });
aGATE2	    = assign({ { 303 } });
aGCUT       = assign({ { 5 } });
aDUMMY	    = assign({ { 4 } });
aNSEL        = assign({ { 6 } });
aPSEL        = assign({ { 7 } });
aSDCON      = assign({ { 10 } });
tSDCON	    = assign_text({ { 10 } });
aSDCON2      = assign({ { 310 } });
tSDCON2	    = assign_text({ { 310 } });
aVSD        = assign({ { 11 } });
aVSD2        = assign({ { 311 } });
aVSD12       = assign({ { 324 } });
aVG         = assign({ { 12 } });
aVG2         = assign({ { 312 } });
aVG12        = assign({ { 352 } });
aM0         = assign({ { 20 } });
tM0	    = assign_text({ { 20 } });
aV0         = assign({ { 22 } });
aM1         = assign({ { 25 } });
tM1	    = assign_text({ { 25 } });
aV1	    = assign({ { 27 } });
aM2	    = assign({ { 30 } });
tM2	    = assign_text({ { 30 } });
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




CONNECT_DB : connect_database = NULL_CONNECT_DATABASE;

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aBPR, aSDCON }, aVBPR, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aBPR, aSDCON2 }, aVBPR2, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aSDCON, aM0 }, aVSD, NONE, SHIELDED_OVERLAP }} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aSDCON2, aM0 }, aVSD2, NONE, SHIELDED_OVERLAP }} );

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

gPOLY = ( ( aGATE ) ) not aGCUT;
gPOLY2 = ( ( aGATE2 ) ) not aGCUT;
gCHANNEL = gPOLY and aACT;
gCHANNEL2 = gPOLY2 and aACT2;
gN_GATE = gCHANNEL2 and aNSEL; 
gP_GATE = gCHANNEL and aPSEL;

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY, aM0 }, aVG, NONE, SHIELDED_OVERLAP }} ); 

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY, aACT }}} ); 

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aGATE, aVG }}} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY2, aM0 }, aVG2, NONE, SHIELDED_OVERLAP }} ); 

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gPOLY2, aACT2 }}} ); 

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ aGATE2, aVG2 }}} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gN_GATE, aVG2 }}} );

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gP_GATE, aVG }}} );

gP_GATE_or_gN_GATE = gP_GATE or gN_GATE;

aVG_or_aVG2 = aVG or aVG2;

CONNECT_DB = incremental_connect( CONNECT_DB, {{{ gP_GATE_or_gN_GATE , aVG_or_aVG2 }}} );


#include "Include/gtA5.bpr.drc.rs"
#include "Include/gtA5.vbpr.drc.rs"
#include "Include/gtA5.vbpr2.drc.rs"
#include "Include/gtA5.gate.drc.rs"
#include "Include/gtA5.gate2.drc.rs"
#include "Include/gtA5.dummy.drc.rs"
#include "Include/gtA5.gcut.drc.rs"
#include "Include/gtA5.vg.drc.rs"
#include "Include/gtA5.vg2.drc.rs"
#include "Include/gtA5.act.drc.rs"
#include "Include/gtA5.act2.drc.rs"
#include "Include/gtA5.npsel.drc.rs"
#include "Include/gtA5.sdcon.drc.rs"
#include "Include/gtA5.sdcon2.drc.rs"
#include "Include/gtA5.vsd.drc.rs"
#include "Include/gtA5.vsd2.drc.rs"
#include "Include/gtA5.vsd12.drc.rs"
#include "Include/gtA5.vg12.drc.rs"
#include "Include/gtA5.m0.drc.rs"
#include "Include/gtA5.v0.drc.rs"
#include "Include/gtA5.m1.drc.rs"
#include "Include/gtA5.v1.drc.rs"
#include "Include/gtA5.m2.drc.rs"
#include "Include/gtA5.v2.drc.rs"
#include "Include/gtA5.m3.drc.rs"
#include "Include/gtA5.v3.drc.rs"
#include "Include/gtA5.m4.drc.rs"
#include "Include/gtA5.v4.drc.rs"
#include "Include/gtA5.m5.drc.rs"
#include "Include/gtA5.v5.drc.rs"
#include "Include/gtA5.m6.drc.rs"


