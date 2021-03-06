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
 fn test_mul() {
    	assert_eq!(Val::Int(5).mul(Val::Int(3)), Val::from_int(15));
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

  #[test]
 fn test_str_from(){
 	assert_eq!(Val::from_str("hello").str(), "hello".to_string());
 }

   #[test]
 fn test_int_from(){
 	assert_eq!(Val::from_int(1).int(), 1);
 }

 #[test]
 fn test_len(){
 	assert_eq!(Val::Bool(true).len(), 1);
 }

 #[test]
 fn test_from_ivec(){
 	assert_eq!(Val::repr(&Val::from_ivec(vec![2, 3, 4])), "[2,3,4,]".to_string());
 }

 #[test]
 fn test_push(){
 	assert_eq!(Val::repr(&Val::from_ivec(vec![2, 3, 4]).push(Val::Int(7))), "[2,3,4,7,]".to_string());
 }

  #[test]
 fn test_pop(){
 	assert_eq!(Val::repr(&Val::from_ivec(vec![2, 3, 4]).pop()), "4".to_string());
 }

 #[test]
 fn test_bool(){
 	assert_eq!(Val::Int(8).bool(), true);
 }

 #[test]
 fn test_get(){
 	assert_eq!(Val::from_str("hello").get(2 as usize).str(), "l");
 }
