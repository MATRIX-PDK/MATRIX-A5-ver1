rBPR_1 @= { @ "BRP.1 : BPR vertical width = 24 nm";
   gBPR1_check = internal1( aBPR, == 0.024, extension = NONE, direction = VERTICAL); 
	aBPR not gBPR1_check;
}

rBPR_2 @= { @ "BPR.2 : Minimum vertical spacing between BPR layers >= 12 nm";
	external1(aBPR, < 0.012, extension = NONE, direction = VERTICAL); 

}

rBPR_3 @= { @ "BPR.3 : BPR must be continuous";
	external1(aBPR, < 10, direction = HORIZONTAL, extension = NONE);
}

rBPR_4 @= { @ "BPR.4 : BPR may not bend";
	not_rectangles(aBPR);
}

