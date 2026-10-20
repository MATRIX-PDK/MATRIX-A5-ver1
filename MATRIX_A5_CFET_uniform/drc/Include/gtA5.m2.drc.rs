rM2_1 @= { @ "M2.1 : Vertical width of M2 >= 9 nm";
        internal1(aM2, < 0.009, extension = NONE, direction = VERTICAL);
}

rM2_2 @= { @ "M2.2 : Horizontal length of M2 >= 24 nm";
        internal1(aM2, < 0.024, extension = NONE, direction = HORIZONTAL);
}

rM2_3 @= { @ "M2.3 : Vertical spacing between M2 layers >= 7 nm";
        external1(aM2, < 0.007, extension = NONE, direction = VERTICAL);
}

rM2_4 @= { @ "M2.4 : Horizontal (TtT) spacing between M2 layers on same track >= 17 nm";
        external1(aM2, < 0.017, extension = NONE, direction = HORIZONTAL);
}

rM2_5 @= { @ "M2.5 : M2 corner-to-corner spacing >= 15 nm";
        external_corner1( aM2, distance < 0.015, type = CONVEX_TO_CONVEX);
}

rM2_6 @= { @ "M2.6 : M2 must not bend";
        not_rectangles(aM2);
}
