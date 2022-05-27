use crate::*;
use crate::errors::{ERR_101, ERR_102};



struct Schema{
	category: String,
	aloccated_quantity: U128,
	total_quantity: U128,
	initial_release: U128, //releases should be a fraction
	cliff_release: U128,	
	final_release: U128,
    initial_timestamp: U128,
	cliff_delta: U128,
	final_delta: U128,
	curve_type: CurveType
}


enum CurveType{
	Linear
}

impl Schema {    
    pub fn new(
        &mut self,
        category: String,
        total_quantity: U128,
        initial_release: U128, //releases should be a fraction
        cliff_release: U128,	
        final_release: U128,
        initial_timestamp: U128,
        cliff_delta: U128,
        final_delta: U128,
        curve_type: CurveType
    ) -> Self {
        
        assert_eq!((initial_release.0 + cliff_release.0 + final_release.0), FRACTION_BASE, '{}', ERR_101)
        //TODO: 
        // delta um maior que o outro 
        // validar, criar erros 
        Self{
            category,
            aloccated_quantity: U128(0),
            total_quantity,
            initial_release,
            cliff_release,
            final_release,
            initial_timestamp,
            cliff_delta,
            final_delta,
            curve_type
        }
    }
}

impl Contract{
    pub fn new_schema(
        &mut self,
        category: String,
        total_quantity: U128,
        initial_release: U128, //releases should be a fraction
        cliff_release: U128,	
        final_release: U128,
        initial_timestamp: U128,
        cliff_delta: U128,
        final_delta: U128,
        curve_type: CurveType
    ){
        //TODO:
        // Dever de casa -> https://doc.rust-lang.org/book/ch06-00-enums.html
        //assert_eq!() //-> existe um schema com esse nome no lupmap? 
        //assert_ne!()

        //acessando a variavel shcemas do contracrt 
        // match self.schemas.get(&category){     //retorna uma option, eh um enum, some ou none
        //     Some(x) => panic!("{}", ERR_102):
        //     _ =>(); // unerline eh qualquer outra opcao 
        // }  

        // if let Some(x) = self.schemas.get(&category) {
        //    panic!("{}", ERR_102)
        // }
        
        assert!(!self.schemas.contains_key(&category), "{}", ERR_102);
        

        let schema = Schema::new(
            category.clone(),
            total_quantity,
            initial_release,
            cliff_release,
            final_release,
            initial_timestamp,
            cliff_delta,
            final_delta,
            curve_type
        );

        self.schemas.insert(&category, &schema);

    }

}
    


    //view_schema_data(id) → show schema data

    