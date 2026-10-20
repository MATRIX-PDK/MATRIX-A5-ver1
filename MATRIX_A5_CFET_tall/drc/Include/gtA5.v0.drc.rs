rV0_1 @= { @ "V0.1 : Exact horizontal width of V0 = 9 nm";
       gV01_check = internal1( aV0, == 0.009, extension = NONE, direction = HORIZONTAL);
       aV0 not gV01_check;
}

rV0_2 @= { @ "V0.2 : Exact vertical width of V0 = 9 nm";
       gV02_check = internal1( aV0, == 0.009, extension = NONE, direction = VERTICAL);
       aV0 not gV02_check;
}

rV0_3 @= { @ "V0.3 : V0 must be rectangle";
        not_rectangles(aV0);
}

rV0_4 @= { @ "V0.4 : V0 must not be outside M0";
        not_inside( aV0, aM0, include_touch = ALL);
}

rV0_5 @= { @ "V0.5 : V0 must not be outside M1";
        not_inside(aV0, aM1, include_touch = ALL);
}

rV0_6 @= { @ "V0.6 : V0 corner-to-corner spacing >= 18 nm";
        external_corner1( aV0, distance < 0.018, type = CONVEX_TO_CONVEX);
}

// V0.7 minimum spacing of V0 on the same M0 track
rV0_7 @= { @ "V0.7 : Minimum spacing between V0 on the same M0 track >= 14nm";
        external1( aV0, < 0.014, extension = NONE, direction = HORIZONTAL);
}

rV0_8 @= { @ "V0.M0.1 : V0 enclosure by M0 on opposite sides in horizontal direction >= 4 nm";
        enclose( aV0, aM0, < 0.004, extension = NONE, direction = HORIZONTAL);
}

rV0_9 @= { @ "V0.M1.1 : V0 enclosure by M1 on opposite sides in vertical direction >= 3 nm";
        enclose(aV0, aM1, < 0.003, extension = NONE, direction = VERTICAL);
}

rV0_10 @= { @ "V0.7 : Vertical edges of V0 should align with M1";
        gV0_vert_edges = angle_edge( aV0, angles = 90);
        not_inside_touching_edge( gV0_vert_edges, aM1);
}

rV0_11 @= { @ "V0.8 : Horizontal edges of V0 should align with M0";
        gV0_horz_edges = angle_edge( aV0, angles = 0);
        not_inside_touching_edge( gV0_horz_edges, aM0);
}

