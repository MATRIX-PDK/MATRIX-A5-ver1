rACT_1 @= { @ "ACT.1 : Minimum vertical width of ACT >= 11 nm";
        internal1(aACT, <0.011, extension = NONE, direction = VERTICAL);
}

rACT_2 @= { @ "ACT.2 : Minimum horizontal width of ACT >= 78 nm";
        internal1(aACT, <0.078, extension = NONE, direction = HORIZONTAL);
}

rACT_3 @= { @ "ACT.3 : Vertical spacing between ACT >= 30 nm";
        external1(aACT, <0.03, extension = RADIAL, direction = VERTICAL);
}



rACT_6 @= { @ "ACT.6 : ACT must end inside a DUMMY layer";
    gACT_vert_edges = angle_edge( aACT, angles = 90);
    not_interacting_edge( gACT_vert_edges, aDUMMY);
}

rACT_7 @= { @ "ACT.BPR.1 : Vertical spacing between ACT and BPR >= 10 nm";
        external2(aACT, aBPR, < 0.010, extension = NONE, direction = VERTICAL);
}

