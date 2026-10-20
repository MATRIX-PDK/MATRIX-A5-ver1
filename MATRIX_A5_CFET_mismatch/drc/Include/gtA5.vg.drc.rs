rVG_1 @= { @ "VG.1 : Exact horizontal width of VG = 9 nm";
        gVG1_check = internal1( aVG, == 0.009, extension = NONE, direction = HORIZONTAL);
        aVG not gVG1_check;
}

rVG_2 @= { @ "VG.2 : Exact vertical width of VG = 9 nm";
        gVG2_check = internal1( aVG, == 0.009, extension = NONE, direction = VERTICAL);
        aVG not gVG2_check;
}

rVG_3 @= { @ "VG.3 : VG must not interact with GCUT or DUMMY";
        gVG3_check = or(aGCUT, aDUMMY);
        interacting(aVG, gVG3_check, include_touch = ALL);
}

rVG_4 @= { @ "VG.4 : VG may not exist without GATE";
        not_interacting(aVG, aGATE, include_touch = NONE);
}

rVG_5 @= { @ "VG.5 : VG must completely lie inside GATE and its vertical edges must coincide with those of GATE";
        not_inside(aVG, aGATE, include_touch = ALL);
}



rVG_7 @= { @ "VG.7 : VG must be rectangle";
        not_rectangles(aVG);
}




rVG_9 @= { @ "VG.M0.1 : VG must completely lie inside M0";
        not_inside(aVG, aM0, include_touch = ALL);
}

rVG_12 @= { @ "VG.ACT.1 : VG and ACT vertical spacing >= 6 nm";
        external2( aVG, aACT, distance < 0.006, extension = RADIAL, direction = VERTICAL);
}
