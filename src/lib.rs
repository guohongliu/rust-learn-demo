mod spring;
use spring::{BeanRegistry, BeanAFactory, BeanBFactory, BeanA, BeanB};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn main() {
        // 创建Bean注册中心
        let mut registry = BeanRegistry::new();

        // 注册Bean工厂
        registry.register_bean_factory("BeanA", Box::new(BeanAFactory));
        registry.register_bean_factory("BeanB", Box::new(BeanBFactory));

        // 获取BeanA，会触发BeanA和BeanB的创建，演示循环依赖的解决
        let bean_a = registry.get_bean("BeanA").unwrap();
        println!("成功获取到{}", bean_a.lock().unwrap().get_name());

        // 验证依赖是否正确注入
        if let Some(a) = bean_a.lock().unwrap().downcast_ref::<BeanA>() {
            if let Some(b) = &a.b {
                let res1 = b.lock().unwrap();
                let res_str = res1.get_name();
                println!("BeanA的依赖是{}", res_str);

            }
        }

        let bean_b = registry.get_bean("BeanB").unwrap();
        println!("成功获取到{}", bean_a.lock().unwrap().get_name());
        if let Some(b) = bean_b.lock().unwrap().downcast_ref::<BeanB>() {
            if let Some(a) = &b.a {
                let res1 = a.lock().unwrap();
                let res_str = res1.get_name();
                println!("BeanA的依赖是{}", res_str);
            }
        }

    }
}
