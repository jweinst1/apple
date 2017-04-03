use val::Val;


#[test]
fn test_int() {
	assert_eq!(Val::Int(1).int(), 1);
}


 #[test]
 fn test_plus() {
    	assert_eq!(Val::Int(5).plus(Val::Int(3)).int(), 8);
 }
