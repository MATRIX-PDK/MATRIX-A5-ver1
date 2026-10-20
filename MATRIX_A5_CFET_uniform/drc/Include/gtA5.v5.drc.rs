rV5_1 @= { @ "V5.1 : Horizontal width of V5 >= 21 nm";
       internal1( aV5, < 0.021, extension = NONE, direction = HORIZONTAL);
}

rV5_2 @= { @ "V5.2 : Vertical height of V5 >= 38 nm";
       internal1( aV5, < 0.038, extension = NONE, direction = VERTICAL);
}

// not_inside_touching_edge + angle_edge
rV5_3 @= { @ "V5.3 : Vertical edges of V5 should align with M5";
        gV5_vert_edges = angle_edge( aV5, angles = 90);
        not_inside_touching_edge( gV5_vert_edges, aM5);
}

rV5_4 @= { @ "V5.4 : Horizontal edges of V5 should align with M6";
        gV5_horz_edges = angle_edge( aV5, angles = 0);
        not_inside_touching_edge( gV5_horz_edges, aM6);
}


// V5.6 minimum corner-to-corner spacing
rV5_6 @= { @ "V5.6 : V5 corner-to-corner spacing >= 27 nm";
        external_corner1( aV5, distance < 0.027, type = CONVEX_TO_CONVEX);
}

rV5_7 @= { @ "V5.7 : V5 must interact with M5";
        not_interacting( aV5, aM5, include_touch = NONE);
}

rV5_8 @= { @ "V5.8 : V5 must interact with M6";
        not_interacting( aV5, aM6, include_touch = NONE);
}


rV5_9 @= { @ "V5.M5.EN : V5 enclosure by M5 on opposite sides in the vertical direction >= 6 nm";
        enclose( aV5, aM5, < 0.006, extension = NONE, direction = VERTICAL);
}

rV5_10 @= { @ "V5.M6.EN : V5 enclosure by M6 on opposite sides in the horizontal direction >= 6 nm";
        enclose( aV5, aM6, < 0.006, extension = NONE, direction = HORIZONTAL);
}
