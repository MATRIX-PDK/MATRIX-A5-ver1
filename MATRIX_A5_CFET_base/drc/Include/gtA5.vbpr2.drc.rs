rVBPR2_1 @= { @ "VBPR2.1 : Vertical width of VBPR2 >= 10 nm";
        internal1( aVBPR2, < 0.010, extension = NONE, direction = VERTICAL);
}

rVBPR2_2 @= { @ "VBPR2.2 : HORIZONTAL width of VBPR2 =12 nm";
        gVBPR22_check = internal1( aVBPR2, == 0.012, extension = NONE, direction = HORIZONTAL);
        aVBPR2 not gVBPR22_check;
}

rVBPR2_3 @= { @ "VBPR2.3 : VBPR2 must not interact with GCUT";
        interacting( aVBPR2, aGCUT, include_touch = NONE);
}

rVBPR2_4 @= { @ "VBPR2.4 : VBPR2 may not lie outside SDCON";
        not_inside( aVBPR2, aSDCON2, include_touch = ALL);
}
