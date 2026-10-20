rGCUT_1 @= { @ "GCUT.1 : GCUT vertical width >= 12 nm";
        internal1( aGCUT, distance < 0.012, extension = NONE, direction = VERTICAL);
}

rGCUT_2 @= { @ "GCUT.2 : GCUT horizontal length >= 39 nm";
        internal1( aGCUT, distance < 0.039, extension = NONE, direction = HORIZONTAL);
}



rGCUT_5 @= { @ "GCUT.5 : GCUT may not bend";
        not_rectangles( aGCUT);
}

rGCUT_6 @= { @ "GCUT.6 : GCUT may not exist without GATE";
        not_interacting( aGCUT, aGATE, include_touch = NONE);
}

rGCUT_7 @= { @ "GCUT.7 : GCUT vertical edge may not lie inside or coincide with GATE vertical edge";
        gGCUT_edge = angle_edge( aGCUT, angles = {90.0});
        gGATE1 = aGATE not aDUMMY;
        interacting_edge( gGCUT_edge, gGATE1);
}

rGCUT_8 @= { @ "GCUT.8 : GCUT may not interact with ACT";
        interacting( aGCUT, aACT, include_touch = ALL);
}

rGCUT_9 @= { @ "GCUT.ACT.1 : Vertical spacing between GCUT and ACT >= 9 nm";
        external2( aGCUT, aACT, distance < 0.009, extension = NONE, direction = VERTICAL);
}

rGCUT_10 @= { @ "GCUT.10 : GCUT may not exist without GATE2";
        not_interacting( aGCUT, aGATE2, include_touch = NONE);
}

rGCUT_11 @= { @ "GCUT.11 : GCUT vertical edge may not lie inside or coincide with GATE vertical edge";
        gGCUT_edge = angle_edge( aGCUT, angles = {90.0});
        gGATE21 = aGATE2 not aDUMMY;
        interacting_edge( gGCUT_edge, gGATE21);
}

rGCUT_12 @= { @ "GCUT.12 : GCUT may not interact with ACT";
        interacting( aGCUT, aACT2, include_touch = ALL);
}

rGCUT_13 @= { @ "GCUT.ACT.13 : Vertical spacing between GCUT and ACT >= 9 nm";
        external2( aGCUT, aACT2, distance < 0.009, extension = NONE, direction = VERTICAL);
}
