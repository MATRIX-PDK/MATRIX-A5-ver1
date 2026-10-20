rSDCON_1 @= { @ "SDCON.1 : Horizontal width of SDCON = 12 nm";
        gSDCON1_check = internal1( aSDCON, == 0.012, extension = NONE, direction = HORIZONTAL);
        aSDCON not gSDCON1_check;
}

// only for SDCON on different net, exceptions for BPR connection to VDD/VSS
rSDCON_2 @= { @ "SDCON.2 : Vertical (TtT) spacing between SDCON >= 10 nm";
        external1( aSDCON, < 0.01, extension = NONE, direction = VERTICAL, connectivity = DIFFERENT_NET, connect_sequence = CONNECT_DB );
}

rSDCON_3 @= { @ "SDCON.3 : SDCON may not bend";
        not_rectangles( aSDCON);
}

rSDCON_5 @= { @ "SDCON.GATE.1 : Horizontal spacing between SDCON and GATE >= 7 nm";
        external2( aSDCON, aGATE, < 0.007, extension = NONE, direction = HORIZONTAL);
}
