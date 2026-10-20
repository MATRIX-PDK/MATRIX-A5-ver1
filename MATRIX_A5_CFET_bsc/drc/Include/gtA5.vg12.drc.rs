// VG12 — CFET inter-tier gate via (GATE <-> GATE2). 9 nm cut.

rVG12_1 @= { @ "VG12.1 : Exact horizontal width of VG12 = 9 nm";
        gVG121_check = internal1( aVG12, == 0.009, extension = NONE, direction = HORIZONTAL);
        aVG12 not gVG121_check;
}

rVG12_2 @= { @ "VG12.2 : Exact vertical width of VG12 = 9 nm";
        gVG122_check = internal1( aVG12, == 0.009, extension = NONE, direction = VERTICAL);
        aVG12 not gVG122_check;
}

rVG12_3 @= { @ "VG12.3 : VG12 must not interact with GCUT or DUMMY";
        gVG123_check = or(aGCUT, aDUMMY);
        interacting(aVG12, gVG123_check, include_touch = ALL);
}

rVG12_4 @= { @ "VG12.4 : VG12 must lie inside both GATE and GATE2 (inter-tier alignment)";
        gVG124_overlap = aGATE and aGATE2;
        not_inside(aVG12, gVG124_overlap, include_touch = ALL);
}

rVG12_5 @= { @ "VG12.5 : VG12 must be rectangular";
        not_rectangles(aVG12);
}
