rVSD2_1 @= { @ "VSD2.1 : Minimum horizontal width of VSD2 >= 9 nm";
        internal1( aVSD2, < 0.009, extension = NONE, direction = HORIZONTAL);
}

rVSD2_2 @= { @ "VSD2.2 : Minimum vertical width of VSD2 >= 9 nm";
        internal1( aVSD2, < 0.009, extension = NONE, direction = VERTICAL);
}

rVSD2_3 @= { @ "VSD2.3 : VSD2 must be rectangular";
        not_rectangles(aVSD2);
}

rVSD2_4 @= { @ "VSD2.4 : VSD2 must not be outside of SDCON2";
        not_inside( aVSD2, aSDCON2, include_touch = ALL);
}

rVSD2_5 @= { @ "VSD2.5 : VSD2 must land on M0";
        not_interacting( aVSD2, aM0, include_touch = NONE);
}

rVSD2_6 @= { @ "VSD2.SDCON2.1 : VSD2 enclosure by SDCON2 on opposite sides in horizontal direction >= 1 nm";
        enclose( aVSD2, aSDCON2, < 0.001, extension = NONE, direction = HORIZONTAL);
}

rVSD2_7 @= { @ "VSD2.M0.1 : The VSD2 landing on M0 must be at least 9 x 9 nm";
        gVSD2_M0_land = aVSD2 and aM0;
        internal1( gVSD2_M0_land, < 0.009, extension = NONE, direction = HORIZONTAL);
        internal1( gVSD2_M0_land, < 0.009, extension = NONE, direction = VERTICAL);
}
