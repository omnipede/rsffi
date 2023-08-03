pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

// 주의) 반드시 포함되어 있어야 한다.
// Build script 에서 생성한 scaffolding 코드를 포함하는 매크로
uniffi::include_scaffolding!("awesomemodule");
