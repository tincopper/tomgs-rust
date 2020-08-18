
// 定义结构体
// 对于使用println!直接输出对象信息需要使用如下注解
#[derive(Debug)]
pub struct User {
    // 这个字段还可以加pub关键字声明这个字段是对外可见的
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

pub fn user_build() {
    let mut user = User {
        username: String::from("tomgs"),
        email: String::from("123@qq.com"),
        sign_in_count: 1,
        active: true,
    };

    // 格式化输出:q
    println!("user is {:?}", user);
    println!("user is {:#?}", user);

    // 读取属性值
    let email = user.email;
    println!("email: {}", email);

    // 修改属性值
    user.email = String::from("1234@qq.com");
    let email = user.email;
    println!("email: {}", email);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    // 实例化结构体
    #[test]
    fn test_user() {
        println!("123123131");
        user_build();
    }

}