rM1_1 @= { @ "M1.1 : Horizontal width of M1 >= 16 nm";
        internal1(aM1, < 0.016, extension = NONE, direction = HORIZONTAL);
}

rM1_2 @= { @ "M1.2 : Vertical length of M1 >= 25 nm";
        internal1(aM1, < 0.025, extension = NONE, direction = VERTICAL);
}

rM1_3 @= { @ "M1.3 : Horizontal spacing between M1 layers >= 10 nm";
        external1(aM1, < 0.01, extension = NONE, direction = HORIZONTAL);
}

rM1_4 @= { @ "M1.4 : Vertical (TtT) spacing between M1 layers on same track >= 14 nm";
        external1(aM1, < 0.014, extension = NONE, direction = VERTICAL);
}

rM1_5 @= { @ "M1.5 : M1 corner-to-corner spacing >= 10 nm";
        external_corner1( aM1, distance < 0.010, type = CONVEX_TO_CONVEX);
}

rM1_6 @= { @ "M1.6 : M1 must not bend";
        not_rectangles(aM1);
}
