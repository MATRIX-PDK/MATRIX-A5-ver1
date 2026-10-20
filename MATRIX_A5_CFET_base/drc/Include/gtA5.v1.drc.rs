rV1_1 @= { @ "V1.1 : Horizontal width of V1 >= 9 nm";
       internal1( aV1, < 0.009, extension = NONE, direction = HORIZONTAL);
}

rV1_2 @= { @ "V1.2 : Vertical height of V1 >= 9 nm";
       internal1( aV1, < 0.009, extension = NONE, direction = VERTICAL);
}

// not_inside_touching_edge + angle_edge
rV1_3 @= { @ "V1.3 : Vertical edges of V1 should align with M1";
        gV1_vert_edges = angle_edge( aV1, angles = 90);
        not_inside_touching_edge( gV1_vert_edges, aM1);
}

rV1_4 @= { @ "V1.4 : Horizontal edges of V1 should align with M2";
        gV1_horz_edges = angle_edge( aV1, angles = 0);
        not_inside_touching_edge( gV1_horz_edges, aM2);
}


// V1.6 minimum corner-to-corner spacing
rV1_6 @= { @ "V1.6 : V1 corner-to-corner spacing >= 18 nm";
        external_corner1( aV1, distance < 0.018, type = CONVEX_TO_CONVEX);
}

rV1_7 @= { @ "V1.7 : V1 must interact with M1";
        not_interacting( aV1, aM1, include_touch = NONE);
}

rV1_8 @= { @ "V1.8 : V1 must interact with M2";
        not_interacting( aV1, aM2, include_touch = NONE);
}


rV1_9 @= { @ "V1.M1.EN : V1 enclosure by M1 on opposite sides in the vertical direction >= 3 nm";
        enclose( aV1, aM1, < 0.003, extension = NONE, direction = VERTICAL);
}

rV1_10 @= { @ "V1.M2.EN : V1 enclosure by M2 on opposite sides in the horizontal direction >= 4 nm";
        enclose( aV1, aM2, < 0.004, extension = NONE, direction = HORIZONTAL);
}
