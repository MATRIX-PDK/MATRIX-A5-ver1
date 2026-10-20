rSDCON2_1 @= { @ "SDCON2.1 : Horizontal width of SDCON2 = 12 nm";
        gSDCON21_check = internal1( aSDCON2, == 0.012, extension = NONE, direction = HORIZONTAL);
        aSDCON2 not gSDCON21_check;
}

// only for SDCON2 on different net, exceptions for BPR connection to VDD/VSS
rSDCON2_2 @= { @ "SDCON2.2 : Vertical (TtT) spacing between SDCON2 >= 10 nm";
        external1( aSDCON2, < 0.01, extension = NONE, direction = VERTICAL, connectivity = DIFFERENT_NET, connect_sequence = CONNECT_DB );
}

rSDCON2_3 @= { @ "SDCON2.3 : SDCON2 may not bend";
        not_rectangles( aSDCON2);
}

rSDCON2_5 @= { @ "SDCON2.GATE2.1 : Horizontal spacing between SDCON2 and GATE2 >= 7 nm";
        external2( aSDCON2, aGATE2, < 0.007, extension = NONE, direction = HORIZONTAL);
}
