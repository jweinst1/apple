use val::Val;


#[test]
fn test_int() {
	assert_eq!(Val::Int(1).int(), 1);
}

#[test]
fn test_is_int() {
	assert!(Val::Int(1).is_int());
}

 #[test]
 fn test_plus() {
    	assert_eq!(Val::Int(5).plus(Val::Int(3)).int(), 8);
 }

  #[test]
 fn test_sub() {
    	assert_eq!(Val::Int(5).sub(Val::Int(3)).int(), 2);
 }

   #[test]
 fn test_min() {
    	assert_eq!(Val::Int(5).min(Val::Int(3)).int(), 3);
 }

 //string tests

 #[test]
 fn test_str(){
 	assert_eq!(Val::Str("t".to_string()).str(), "t".to_string());
 }

  #[test]
 fn test_str_plus(){
 	assert_eq!(Val::Str("t".to_string()).plus(Val::Str("j".to_string())).str(), "tj".to_string());
 }
