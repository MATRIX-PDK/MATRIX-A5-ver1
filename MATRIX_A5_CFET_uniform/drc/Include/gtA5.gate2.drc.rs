rGATE2_1 @= { @ "GATE2.1 : Exact horizontal width of GATE2 = 12 nm";
        gGATE21_check = internal1( aGATE2, distance == 0.012, extension = NONE, direction = HORIZONTAL);
        aGATE2 not gGATE21_check;
}

// may give false DRC errors if the gates are not aligned
rGATE2_2 @= { @ "GATE2.2 : Exact horizontal spacing of GATE2 = 27 nm";
        gGATE22_check = external1( aGATE2, distance == 0.027, extension = NONE, direction = HORIZONTAL);
        gGATE22_check_extent = layer_extent(aGATE2);
        gGATE22_check_extent not aGATE2 not gGATE22_check;
        //aGATE2 not gGATE22_check;
        //external1( aGATE2, distance == 0.027, extension = NONE, direction = HORIZONTAL);
}


rGATE2_4 @= { @ "GATE2.4 : GATE2 must not bend";
        not_rectangles(aGATE2);
}
