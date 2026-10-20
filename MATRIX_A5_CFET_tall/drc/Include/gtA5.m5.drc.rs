rM5_1 @= { @ "M5.1 : Horizontal width of M5 >= 21 nm";
        internal1(aM5, < 0.021, extension = NONE, direction = HORIZONTAL);
}

rM5_2 @= { @ "M5.2 : Vertical length of M5 >= 42 nm";
        internal1(aM5, < 0.042, extension = NONE, direction = VERTICAL);
}

rM5_3 @= { @ "M5.3 : Horizontal spacing between M5 layers >= 21 nm";
        external1(aM5, < 0.021, extension = NONE, direction = HORIZONTAL);
}

rM5_4 @= { @ "M5.4 : Vertical (TtT) spacing between M5 layers on same track >= 30 nm";
        external1(aM5, < 0.03, extension = NONE, direction = VERTICAL);
}

rM5_5 @= { @ "M5.5 : M5 corner-to-corner spacing >= 24 nm";
        external_corner1( aM5, distance < 0.024, type = CONVEX_TO_CONVEX);
}

rM5_6 @= { @ "M5.6 : M5 must not bend";
        not_rectangles(aM5);
}
