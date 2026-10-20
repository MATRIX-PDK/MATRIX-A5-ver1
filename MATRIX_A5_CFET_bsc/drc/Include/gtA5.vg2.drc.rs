rVG2_1 @= { @ "VG2.1 : Exact horizontal width of VG2 = 9 nm";
        gVG21_check = internal1( aVG2, == 0.009, extension = NONE, direction = HORIZONTAL);
        aVG2 not gVG21_check;
}

rVG2_2 @= { @ "VG2.2 : Exact vertical width of VG2 = 9 nm";
        gVG22_check = internal1( aVG2, == 0.009, extension = NONE, direction = VERTICAL);
        aVG2 not gVG22_check;
}

rVG2_3 @= { @ "VG2.3 : VG2 must not interact with GCUT or DUMMY";
        gVG23_check = or(aGCUT, aDUMMY);
        interacting(aVG2, gVG23_check, include_touch = ALL);
}

rVG2_4 @= { @ "VG2.4 : VG2 may not exist without GATE";
        not_interacting(aVG2, aGATE2, include_touch = NONE);
}

rVG2_5 @= { @ "VG2.5 : VG2 must completely lie inside GATE and its vertical edges must coincide with those of GATE";
        not_inside(aVG2, aGATE2, include_touch = ALL);
}



rVG2_7 @= { @ "VG2.7 : VG2 must be rectangle";
        not_rectangles(aVG2);
}




rVG2_9 @= { @ "VG2.M0.1 : VG2 must completely lie inside M0";
        not_inside(aVG2, aM0, include_touch = ALL);
}

rVG2_12 @= { @ "VG2.ACT2.1 : VG2 and ACT2 vertical spacing >= 6 nm";
        external2( aVG2, aACT2, distance < 0.006, extension = RADIAL, direction = VERTICAL);
}
