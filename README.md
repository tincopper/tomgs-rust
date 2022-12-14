# tomgs-rust
Rust learning record

# 创建项目
```shell
cd tomgs-rust
cargo new [项目名称]
```

# 添加测试
1、先编写测试文件
```rust
#[cfg(test)]
mod tests {

    #[test]
    fn test_match_mod() {
        let x: u32 = 3;
        match x {
            0 => println!("x = 0"),
            1 | 2 => println!("1 <= x <= 2"),
            3 | 4 | 5 => println!("3 <= x <= 5"),
            _ => println!("x > 5"),
        }
    }
}
```
2、然后再在`lib.rs`当中引入mod
```rust
mod pattern_match_test;
```

