rGATE_1 @= { @ "GATE.1 : Exact horizontal width of GATE = 12 nm";
        gGATE1_check = internal1( aGATE, distance == 0.012, extension = NONE, direction = HORIZONTAL);
        aGATE not gGATE1_check;
}

// may give false DRC errors if the gates are not aligned
rGATE_2 @= { @ "GATE.2 : Exact horizontal spacing of GATE = 27 nm";
        gGATE2_check = external1( aGATE, distance == 0.027, extension = NONE, direction = HORIZONTAL);
        gGATE2_check_extent = layer_extent(aGATE);
        gGATE2_check_extent not aGATE not gGATE2_check;
        //aGATE not gGATE2_check;
        //external1( aGATE, distance == 0.027, extension = NONE, direction = HORIZONTAL);
}


rGATE_4 @= { @ "GATE.4 : GATE must not bend";
        not_rectangles(aGATE);
}
