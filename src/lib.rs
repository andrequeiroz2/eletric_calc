use std::io;

const CONST_OPERATION: &str = "operation_type";
const CONST_UNKNOWN1: &str = "unknown_1";
const CONST_UNKNOWN2: &str = "unknown_2";

#[derive(Clone, PartialEq)]
enum UnknownType{
    P,
    V,
    I,
    R,   
}
impl UnknownType {
    fn from_str(s: &str, e: &str) -> Result<Self, io::Error> {
      match s {
        "P" => Ok(UnknownType::P),
        "V" => Ok(UnknownType::V),
        "I" => Ok(UnknownType::I),
        "R" => Ok(UnknownType::R),
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid {}. Values accepted: P, V, I, R", e))),
      }
    }
}

struct Unknowns{
    unknown_1: UnknownType,
    unknown_2: UnknownType,
}
impl Unknowns{
    fn validate(&self) -> Result<(), io::Error>{
        
        if self.unknown_1 == self.unknown_2{
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Types of unknowns cannot be equal"))?
        }

        Ok(())
    }
} 

pub struct EletricCalc{
    pub operation_type: String, 
    pub unknown_1: String, 
    pub unknown_2: String, 
    pub value_unknown_1: f64, 
    pub value_unknown_2: f64
}

impl EletricCalc {

    pub fn calc(self) -> Result<String, io::Error> {
        
        let operation_type = UnknownType::from_str(&self.operation_type.to_uppercase(), &CONST_OPERATION)?;
        let unknown_1 = UnknownType::from_str(&self.unknown_1.to_uppercase(), &CONST_UNKNOWN1)?;
        let unknown_2 = UnknownType::from_str(&self.unknown_2.to_uppercase(), &CONST_UNKNOWN2)?;
        
        let unknowns = Unknowns{
            unknown_1: unknown_1.clone(), 
            unknown_2: unknown_2.clone(),
        };

        _ = unknowns.validate()?;
    
        let result = self.calc_result(operation_type, unknown_1, unknown_2);
        
        Ok(result)

    }

    fn calc_result(&self, operation_type: UnknownType, unknown_1: UnknownType, unknown_2: UnknownType)-> String {
            
        match operation_type {
            UnknownType::P =>{

                if (unknown_1 == UnknownType::V || unknown_1 == UnknownType::I) && (unknown_2 == UnknownType::V || unknown_2 == UnknownType::I){
                
                    return format!("{}{}", self.value_unknown_1 * self.value_unknown_2, String::from("W"))
                
                }
                

                if unknown_1 == UnknownType::I && unknown_2 == UnknownType::R {
                
                    return format!("{}{}", ( self.value_unknown_1.powf(2.0)) * self.value_unknown_2, String::from("W"))
                
                }else if unknown_1 == UnknownType::R && unknown_2 == UnknownType::I{
                    
                    return format!("{}{}", ( self.value_unknown_2.powf(2.0)) * self.value_unknown_1, String::from("W"))

                }
                
                
                if unknown_1 == UnknownType::V && unknown_2 == UnknownType::R {
                    
                    return format!("{}{}", ( self.value_unknown_1.powf(2.0)) / self.value_unknown_2, String::from("W"))

                } else if unknown_1 == UnknownType::R && unknown_2 == UnknownType::V {
                    
                    return format!("{}{}", ( self.value_unknown_2.powf(2.0)) / self.value_unknown_1, String::from("W"))
                
                }else {

                    return format!("Invalid operation.")
                }
            }
            
            UnknownType::I =>{

                if unknown_1 == UnknownType::P && unknown_2 == UnknownType::V {

                    return format!("{}{}", self.value_unknown_1 / self.value_unknown_2, self.operation_type)
                
                }else if unknown_1 == UnknownType::V && unknown_2 == UnknownType::P{
                    
                    return format!("{}{}", self.value_unknown_2 / self.value_unknown_1, self.operation_type)

                }

                
                if unknown_1 == UnknownType::V && unknown_2 == UnknownType::R {
                    
                    return format!("{}{}", self.value_unknown_1 / self.value_unknown_2, self.operation_type)

                }else if unknown_1 == UnknownType::R && unknown_2 == UnknownType::V{

                    return format!("{}{}", self.value_unknown_2 / self.value_unknown_1, self.operation_type)

                }

                if unknown_1 == UnknownType::P && unknown_2 == UnknownType::R {
                    
                    let result = (self.value_unknown_1 / self.value_unknown_2).sqrt();

                    return format!("{}{}", result, self.operation_type)
                    

                }else if unknown_1 == UnknownType::R && unknown_2 == UnknownType::P{

                    let result = (self.value_unknown_2 / self.value_unknown_1).sqrt();

                    return format!("{}{}", result, self.operation_type)

                }else{

                    return format!("Invalid operation.")
                
                }
            }

                
            UnknownType::V =>{
                
                if (unknown_1 == UnknownType::I || unknown_1 == UnknownType::R) && (unknown_2 == UnknownType::I || unknown_2 == UnknownType::R){
                
                    return format!("{}{}", self.value_unknown_1 * self.value_unknown_2, self.operation_type)
                
                }

                if unknown_1 == UnknownType::P && unknown_2 == UnknownType::I{
                    
                    return format!("{}{}", self.value_unknown_1 / self.value_unknown_2, self.operation_type)
                
                }else if unknown_1 == UnknownType::I && unknown_2 == UnknownType::P{

                    return format!("{}{}", self.value_unknown_2 / self.value_unknown_1, self.operation_type)
                    
                }
                
                if unknown_1 == UnknownType::P && unknown_2 == UnknownType::R{
                
                    let result = (self.value_unknown_1 * self.value_unknown_2).sqrt();

                    return format!("{}{}", result, self.operation_type)

                }else if unknown_1 == UnknownType::R && unknown_2 == UnknownType::P{

                    let result = (self.value_unknown_2 * self.value_unknown_1).sqrt();

                    return format!("{}{}", result, self.operation_type)
                    
                }else{

                    return format!("Invalid operation.")
                }

            }
                        
            UnknownType::R =>{
                if unknown_1 == UnknownType::P && unknown_2 == UnknownType::I{
                    
                    return format!("{}{}", self.value_unknown_1 / self.value_unknown_2.powf(2.0), self.operation_type)
                
                }else if unknown_1 == UnknownType::I && unknown_2 == UnknownType::P{
                    
                    return format!("{}{}", self.value_unknown_2 / self.value_unknown_1.powf(2.0), self.operation_type)

                }

                if unknown_1 == UnknownType::V && unknown_2 == UnknownType::P{
                    
                    return format!("{}{}", self.value_unknown_1.powf(2.0) / self.value_unknown_2, self.operation_type)

                }else if unknown_1 == UnknownType::P && unknown_2 == UnknownType::V{
                    
                    return format!("{}{}", self.value_unknown_2.powf(2.0) / self.value_unknown_1, self.operation_type)

                }

                if unknown_1 == UnknownType::V && unknown_2 == UnknownType::I{

                    return format!("{}{}", self.value_unknown_1 / self.value_unknown_2, self.operation_type)

                }else if unknown_1 == UnknownType::I && unknown_2 == UnknownType::V{
                    
                    return format!("{}{}", self.value_unknown_2 / self.value_unknown_1, self.operation_type)
                
                }else {
                    
                    return format!("Invalid operation.")

                }
            }
        }  
    }      
}


#[cfg(test)]
mod tests {
    use std::io;

    use crate::EletricCalc;

    #[test]
    fn calc_power_v_i(){
        let result = EletricCalc{
            operation_type: String::from("P"),
            unknown_1: String::from("V"),
            unknown_2: String::from("I"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("4W"));
            },
            Err(_) => {
                assert!(false)
            },  
        };
    }
    #[test]
    fn calc_power_v_r(){
        let result = EletricCalc{
            operation_type: String::from("P"),
            unknown_1: String::from("R"),
            unknown_2: String::from("V"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("2W"));
            },
            Err(_) => {
                assert!(false)
            },  
        };
    }
    #[test]
    fn calc_power_i_r(){
        let result = EletricCalc{
            operation_type: String::from("p"),
            unknown_1: String::from("R"),
            unknown_2: String::from("i"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("8W"));
            },
            Err(_) => {
                assert!(false)
            },   
        };
    }
    #[test]
    fn calc_amps_i_r(){
        let result = EletricCalc{
            operation_type: String::from("I"),
            unknown_1: String::from("V"),
            unknown_2: String::from("P"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("1I"));
            },
            Err(_) => {
                assert!(false)
            },  
        };
    }
    #[test]
    fn calc_amps_p_r(){
        let result = EletricCalc{
            operation_type: String::from("I"),
            unknown_1: String::from("R"),
            unknown_2: String::from("P"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("1I"));
            },
            Err(_) => {
                assert!(false)
            },   
        };
    }
    #[test]
    fn calc_amps_v_r(){
        let result = EletricCalc{
            operation_type: String::from("I"),
            unknown_1: String::from("R"),
            unknown_2: String::from("V"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("1I"));
            },
            Err(_) => {
                assert!(false)
            },    
        };
    }
    #[test]
    fn calc_ohms_v_r(){
        let result = EletricCalc{
            operation_type: String::from("R"),
            unknown_1: String::from("I"),
            unknown_2: String::from("V"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("1R"));
            },
            Err(_) => {
                assert!(false)
            },   
        };
    }
    #[test]
    fn calc_ohms_v_p(){
        let result = EletricCalc{
            operation_type: String::from("R"),
            unknown_1: String::from("V"),
            unknown_2: String::from("P"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("2R"));
            },
            Err(_) => {
                assert!(false)
            },  
        };
    }
    #[test]
    fn calc_ohms_i_p(){
        let result = EletricCalc{
            operation_type: String::from("R"),
            unknown_1: String::from("I"),
            unknown_2: String::from("P"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("0.5R"));
            },
            Err(_) => {
                assert!(false)
            },   
        };
    }
    #[test]
    fn calc_volts_v_p(){
        let result = EletricCalc{
            operation_type: String::from("V"),
            unknown_1: String::from("I"),
            unknown_2: String::from("R"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("4V"));
            },
            Err(_) => {
                assert!(false)
            },   
        };
    }
    #[test]
    fn calc_volts_i_p(){
        let result = EletricCalc{
            operation_type: String::from("V"),
            unknown_1: String::from("I"),
            unknown_2: String::from("P"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("1V"));
            },
            Err(_) => {
                assert!(false)
            },   
        };
    }
    #[test]
    fn calc_volts_r_p(){
        let result = EletricCalc{
            operation_type: String::from("V"),
            unknown_1: String::from("R"),
            unknown_2: String::from("P"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(v) => {
                assert_eq!(v, String::from("2V"));
            },
            Err(_) => {
                assert!(false)
            },    
        };
    }
    #[test]
    fn operation_error(){
        let result = EletricCalc{
            operation_type: String::from("V"),
            unknown_1: String::from("R"),
            unknown_2: String::from("R"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(_) => {
                    assert!(false)  
            },
            Err(_) => {
                assert!(true) 
            },  
        };
    }
    #[test]
    fn unknown_1_error(){
        let result = EletricCalc{
            operation_type: String::from("V"),
            unknown_1: String::from("K"),
            unknown_2: String::from("R"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(_) => {
                assert!(false)
            },
            Err(_) => {
                assert!(true) 
            },  
        };
    }
    #[test]
    fn unknown_2_error(){
        let result = EletricCalc{
            operation_type: String::from("V"),
            unknown_1: String::from("R"),
            unknown_2: String::from("K"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(_) => {
                assert!(false)
            },
            Err(_) => {
                assert!(true) 
            },  
        };
    }
    #[test]
    fn unknown_equals_error(){
        let result = EletricCalc{
            operation_type: String::from("V"),
            unknown_1: String::from("R"),
            unknown_2: String::from("R"),
            value_unknown_1: 2.0,
            value_unknown_2: 2.0
        }.calc();

        match result {
            Ok(_) => {
                assert!(false)
            },
            Err(_) => {
                assert!(true) 
            },  
        };
    }
}