rVSD_1 @= { @ "VSD.1 : Horizontal width of VSD = 9 nm";
        gVSD1_check = internal1( aVSD, == 0.009, extension = NONE, direction = HORIZONTAL);
        aVSD not gVSD1_check;
}

rVSD_2 @= { @ "VSD.2 : Vertical width of VSD = 9 nm";
        gVSD2_check = internal1( aVSD, == 0.009, extension = NONE, direction = VERTICAL);
        aVSD not gVSD2_check;
}

rVSD_3 @= { @ "VSD.3 : VSD must be rectangular";
        not_rectangles(aVSD);
}

rVSD_4 @= { @ "VSD.4 : VSD must not be outside of SDCON";
        not_inside( aVSD, aSDCON, include_touch = ALL);
}

rVSD_5 @= { @ "VSD.5 : VSD must not be outside of M0";
        not_inside( aVSD, aM0, include_touch = ALL);
}

rVSD_6 @= { @ "VSD.SDCON.1 : VSD enclosure by SDCON on opposite sides in horizontal direction >= 1 nm";
        enclose( aVSD, aSDCON, < 0.001, extension = NONE, direction = HORIZONTAL);
}

