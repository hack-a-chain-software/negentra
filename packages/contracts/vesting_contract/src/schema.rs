struct Schema{
	category: String,
	aloccated_quantity: U128,
	toal_quantity: U128,
	initial_release: U18, //releases should be a fraction
	cliff_release: U18,	
	final_release: U18,
    initial_timestamp: U128,
	cliff_delta: U128,
	final_delta: U128,
	curve_type: CurveType
}


enum CurveType{
	Linear
}