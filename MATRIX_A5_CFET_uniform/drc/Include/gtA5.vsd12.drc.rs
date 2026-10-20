// VSD12 — CFET inter-tier S/D via (SDCON <-> SDCON2). 9 nm cut.

rVSD12_1 @= { @ "VSD12.1 : Horizontal width of VSD12 = 9 nm";
        gVSD121_check = internal1( aVSD12, == 0.009, extension = NONE, direction = HORIZONTAL);
        aVSD12 not gVSD121_check;
}

rVSD12_2 @= { @ "VSD12.2 : Vertical width of VSD12 = 9 nm";
        gVSD122_check = internal1( aVSD12, == 0.009, extension = NONE, direction = VERTICAL);
        aVSD12 not gVSD122_check;
}

rVSD12_3 @= { @ "VSD12.3 : VSD12 must be rectangular";
        not_rectangles(aVSD12);
}

rVSD12_4 @= { @ "VSD12.4 : VSD12 must lie inside both SDCON and SDCON2 (inter-tier alignment)";
        gVSD124_overlap = aSDCON and aSDCON2;
        not_inside( aVSD12, gVSD124_overlap, include_touch = ALL);
}
