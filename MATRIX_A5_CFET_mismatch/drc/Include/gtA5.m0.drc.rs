rM0_1 @= { @ "M0.1 : Exact Vertical width of M0 = 9 nm";
	gM01_check = internal1( aM0, == 0.009, extension = NONE, direction = VERTICAL);	
        aM0 not gM01_check;
}

rM0_2 @= { @ "M0.2 : Minimum Horizontal width of M0 >= 16 nm";
	internal1( aM0, < 0.016, extension = NONE, direction = HORIZONTAL);
}

rM0_3 @= { @ "M0.3 : Minimum Vertical spacing of M0 >= 7 nm";
	external1(aM0, < 0.007, extension = RADIAL, direction = VERTICAL); 
}

rM0_4 @= { @ "M0.4 : Minimum Horizontal (TtT) spacing between two M0 layers on the same track >= 5 nm";
        external1(aM0, < 0.005, extension = NONE, intersecting = { }, direction = HORIZONTAL, orientation = {PARALLEL}, look_thru = NONE);
}

rM0_5 @= { @ "M0.5 : M0 corner-to-corner spacing >= 7 nm";
        external_corner1( aM0, distance < 0.007, type = CONVEX_TO_CONVEX);
}


rM0_6 @= { @ "M0.6 : M0 must not bend";
    not_rectangles(aM0);
}


