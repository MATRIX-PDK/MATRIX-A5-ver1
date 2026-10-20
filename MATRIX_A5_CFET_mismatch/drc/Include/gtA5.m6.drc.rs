rM6_1 @= { @ "M6.1 : Vertical width of M6 >= 38 nm";
        internal1(aM6, < 0.038, extension = NONE, direction = VERTICAL);
}

rM6_2 @= { @ "M6.2 : Horizontal length of M6 >= 55 nm";
        internal1(aM6, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rM6_3 @= { @ "M6.3 : Vertical spacing between M6 layers >= 38 nm";
        external1(aM6, < 0.038, extension = NONE, direction = VERTICAL);
}

rM6_4 @= { @ "M6.4 : Horizontal (TtT) spacing between M6 layers on same track >= 55 nm";
        external1(aM6, < 0.055, extension = NONE, direction = HORIZONTAL);
}

rM6_5 @= { @ "M6.5 : M6 corner-to-corner spacing >= 45 nm";
        external_corner1( aM6, distance < 0.045, type = CONVEX_TO_CONVEX);
}

rM6_6 @= { @ "M6.6 : M6 must not bend";
        not_rectangles(aM6);
}
