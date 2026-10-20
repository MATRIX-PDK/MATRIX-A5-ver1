rM3_1 @= { @ "M3.1 : Horizontal width of M3 >= 16 nm";
        internal1(aM3, < 0.016, extension = NONE, direction = HORIZONTAL);
}

rM3_2 @= { @ "M3.2 : Vertical length of M3 >= 26 nm";
        internal1(aM3, < 0.026, extension = NONE, direction = VERTICAL);
}

rM3_3 @= { @ "M3.3 : Horizontal spacing between M3 layers >= 16 nm";
        external1(aM3, < 0.016, extension = NONE, direction = HORIZONTAL);
}

rM3_4 @= { @ "M3.4 : Vertical (TtT) spacing between M3 layers on same track >= 14 nm";
        external1(aM3, < 0.014, extension = NONE, direction = VERTICAL);
}

rM3_5 @= { @ "M3.5 : M3 corner-to-corner spacing >= 16 nm";
        external_corner1( aM3, distance < 0.016, type = CONVEX_TO_CONVEX);
}

rM3_6 @= { @ "M3.6 : M3 must not bend";
        not_rectangles(aM3);
}
