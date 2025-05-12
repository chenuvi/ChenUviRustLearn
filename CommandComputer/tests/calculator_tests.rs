use std::panic::{catch_unwind, AssertUnwindSafe};

// 导入项目中的相关结构体和特征
use command_computer::{Computer, CommandLineComputer};

// 加法测试
#[test]
fn test_addition() {
    let computer = CommandLineComputer;
    assert_eq!(computer.compute("1+2"), 3);
    assert_eq!(computer.compute("10 + 20"), 30);
    assert_eq!(computer.compute("   5   +   5   "), 10);
}

// 减法测试
#[test]
fn test_subtraction() {
    let computer = CommandLineComputer;
    assert_eq!(computer.compute("5-3"), 2);
    assert_eq!(computer.compute("10 - 20"), -10);
    assert_eq!(computer.compute("   100   -   1   "), 99);
}

// 乘法测试
#[test]
fn test_multiplication() {
    let computer = CommandLineComputer;
    assert_eq!(computer.compute("2*3"), 6);
    assert_eq!(computer.compute("10 * 20"), 200);
    assert_eq!(computer.compute("   7   *   8   "), 56);
}

// 除法测试
#[test]
fn test_division() {
    let computer = CommandLineComputer;
    assert_eq!(computer.compute("6/2"), 3);
    assert_eq!(computer.compute("100 / 20"), 5);
    assert_eq!(computer.compute("   81   /   9   "), 9);
}

// 无效表达式测试
#[test]
fn test_invalid_expr() {
    let computer = CommandLineComputer;
    
    // 测试无效字符
    let result = catch_unwind(AssertUnwindSafe(|| {
        computer.compute("1+a");
    }));
    assert!(result.is_err());
    
    // 测试缺少操作数
    let result = catch_unwind(AssertUnwindSafe(|| {
        computer.compute("1+");
    }));
    assert!(result.is_err());
    
    // 测试缺少运算符
    let result = catch_unwind(AssertUnwindSafe(|| {
        computer.compute("12");
    }));
    assert!(result.is_err());
    
    // 测试空字符串
    let result = catch_unwind(AssertUnwindSafe(|| {
        computer.compute("");
    }));
    assert!(result.is_err());
} 