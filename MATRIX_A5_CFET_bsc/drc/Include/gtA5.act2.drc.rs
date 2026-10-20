rACT2_1 @= { @ "ACT2.1 : Minimum vertical width of ACT2 >= 11 nm";
        internal1(aACT2, <0.011, extension = NONE, direction = VERTICAL);
}

rACT2_2 @= { @ "ACT2.2 : Minimum horizontal width of ACT2 >= 78 nm";
        internal1(aACT2, <0.078, extension = NONE, direction = HORIZONTAL);
}

rACT2_3 @= { @ "ACT2.3 : Vertical spacing between ACT2 >= 30 nm";
        external1(aACT2, <0.03, extension = RADIAL, direction = VERTICAL);
}



rACT2_6 @= { @ "ACT2.6 : ACT2 must end inside a DUMMY layer";
    gACT2_vert_edges = angle_edge( aACT2, angles = 90);
    not_interacting_edge( gACT2_vert_edges, aDUMMY);
}

