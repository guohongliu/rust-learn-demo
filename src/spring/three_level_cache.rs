use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

// 定义 Bean 工厂，用于创建 Bean 实例
pub trait BeanFactory {
    fn create(&self, registry: &BeanRegistry) -> Box<dyn AnyBean>;

    fn clone_box(&self) -> Box<dyn BeanFactory>;
}

impl Clone for Box<dyn BeanFactory> {
    fn clone(&self) -> Self {
        self.clone_box() // 调用 trait 对象的 clone_box 方法
    }
}

// 定义 Bean 接口，所有 Bean 都需要实现这个接口
pub trait AnyBean: std::any::Any + Send + Sync {
    fn get_name(&self) -> &str;
    fn set_dependencies(&mut self, registry: &BeanRegistry);
}

// 为 AnyBean 实现 downcast 方法，方便类型转换
impl dyn AnyBean {
    pub fn downcast_ref<T: AnyBean + 'static>(&self) -> Option<&T> {
        (self as &dyn Any).downcast_ref()
    }

    pub fn downcast_mut<T: AnyBean + 'static>(&mut self) -> Option<&mut T> {
        (self as &mut dyn Any).downcast_mut()
    }
}

// 定义具体的Bean: A和B，它们互相依赖
pub struct BeanA {
    name: String,
    pub b: Option<Arc<Mutex<Box<dyn AnyBean>>>>,
}

pub struct BeanB {
    name: String,
    pub a: Option<Arc<Mutex<Box<dyn AnyBean>>>>,
}

impl BeanA {
    fn new() -> Self {
        BeanA {
            name: "BeanA".to_string(),
            b: None,
        }
    }
}

impl BeanB {
    fn new() -> Self {
        BeanB {
            name: "BeanB".to_string(),
            a: None,
        }
    }
}

// 实现AnyBean接口
impl AnyBean for BeanA {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_dependencies(&mut self, registry: &BeanRegistry) {
        // 依赖注入：BeanA需要BeanB
        self.b = Some(registry.get_bean("BeanB").unwrap());
        println!("BeanA注入了BeanB");
    }
}

impl AnyBean for BeanB {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_dependencies(&mut self, registry: &BeanRegistry) {
        // 依赖注入：BeanB需要BeanA
        self.a = Some(registry.get_bean("BeanA").unwrap());
        println!("BeanB注入了BeanA");
    }
}

// 定义具体的Bean工厂
#[derive(Clone)]
pub struct BeanAFactory;

#[derive(Clone)]
pub struct BeanBFactory;


impl BeanFactory for BeanAFactory {
    fn create(&self, _registry: &BeanRegistry) -> Box<dyn AnyBean> {
        Box::new(BeanA::new())
    }

    fn clone_box(&self) -> Box<dyn BeanFactory + 'static> {
        Box::new(self.clone()) // 正确地将 Self 转换为 trait 对象
    }
}

impl BeanFactory for BeanBFactory {
    fn create(&self, _registry: &BeanRegistry) -> Box<dyn AnyBean> {
        Box::new(BeanB::new())
    }

        fn clone_box(&self) -> Box<dyn BeanFactory + 'static> {
        Box::new(self.clone())
    }
}

pub struct BeanRegistry {
    // 一级存储：存储完全初始化的 Bean
    singleton_objects: Mutex<HashMap<String, Arc<Mutex<Box<dyn AnyBean>>>>>,
    // 二级存储：存储提前暴露的未完全初始化的 Bean
    early_singleton_objects:  Mutex<HashMap<String, Arc<Mutex<Box<dyn AnyBean>>>>>,
    // 三级存储：存储 Bean 的工厂
    singleton_factories: Mutex<HashMap<String, Box<dyn BeanFactory>>>,
    // 标记正在创建的 Bean
    singletons_currently_in_creation: Mutex<HashSet<String>>,
    // 注册的 Bean 工厂
    bean_factories: HashMap<String, Box<dyn BeanFactory>>,
}

impl BeanRegistry {
    pub fn new() -> Self {
        BeanRegistry {
            singleton_objects: Mutex::new(HashMap::new()),
            early_singleton_objects: Mutex::new(HashMap::new()),
            singleton_factories: Mutex::new(HashMap::new()),
            singletons_currently_in_creation: Mutex::new(HashSet::new()),
            bean_factories: HashMap::new(),
        }
    }

    // 注册 Bean 工厂
    pub fn register_bean_factory(&mut self, name: &str, factory: Box<dyn BeanFactory>) {
        self.bean_factories.insert(name.to_string(), factory);
    }

    pub fn get_bean(&self, name: &str) -> Option<Arc<Mutex<Box<dyn AnyBean>>>> {
        // 1.先从一级缓存获取
        let singleton_objects = self.singleton_objects.lock().unwrap();
        if let Some(bean) = singleton_objects.get(name) {
            return Some(Arc::clone(bean));
        }
        drop(singleton_objects); // 释放锁

        // 2.检查是否在创建
        let singletons_in_creation = self.singletons_currently_in_creation.lock().unwrap();
        let is_creating = singletons_in_creation.contains(name);
        drop(singletons_in_creation);

        if is_creating {
            // 3. 从二级缓存获取
            let mut early_singletons = self.early_singleton_objects.lock().unwrap();
            if let Some(bean) = early_singletons.get(name) {
                return Some(Arc::clone(bean));
            }
            // 4. 从三级缓存获取并移至二级缓存
            let mut singleton_factories = self.singleton_factories.lock().unwrap();
            if let Some(factory) = singleton_factories.remove(name) {
                // 创建早期对象
                let bean = Arc::new(Mutex::new(factory.create(self)));
                early_singletons.insert(name.to_string(), Arc::clone(&bean));
                return Some(Arc::clone(&bean));
            }
        }

        // 5. 如果不在创建中，则开始创建Bean
        self.create_bean(name)
    }

    // 创建Bean
    fn create_bean(&self, name: &str) -> Option<Arc<Mutex<Box<dyn AnyBean>>>> {
        // 标记为正在创建
        let mut singletons_in_creation = self.singletons_currently_in_creation.lock().unwrap();
        if singletons_in_creation.contains(name) {
            return None; // 防止重复创建
        }
        singletons_in_creation.insert(name.to_string());
        drop(singletons_in_creation);

        // 获取Bean工厂
        let factory: Box<dyn BeanFactory> = {
            let factories = &self.bean_factories;
            factories.get(name)?.clone()
        };

        // 将工厂放入三级缓存
        let mut singleton_factories = self.singleton_factories.lock().unwrap();
        singleton_factories.insert(name.to_string(), factory);
        drop(singleton_factories);

        // 从三级缓存获取早期对象（触发工厂创建）
        let bean = self.get_bean(name)?;

        // // // 这里需要一个可修改的引用进行依赖注入
        // // // 注意：在Rust中共享可修改需要特殊处理，这里使用了Arc内部的可变性
        // let mut_bean: Mutex<Box<dyn AnyBean>> = Arc::try_unwrap(early_bean).ok()?;
        // mut_bean.lock().unwrap().set_dependencies(self);
        // let bean = Arc::new(mut_bean);

        // 直接在 Mutex 中完成依赖注入
        {
            let mut bean_guard = bean.lock().unwrap();
            bean_guard.set_dependencies(self);
        }

        // 从二级缓存移除，放入一级缓存
        let mut early_singletons = self.early_singleton_objects.lock().unwrap();
        early_singletons.remove(name);
        drop(early_singletons);

        let mut singleton_objects = self.singleton_objects.lock().unwrap();
        singleton_objects.insert(name.to_string(), Arc::clone(&bean));
        drop(singleton_objects);

        // 移除创建中标记
        let mut singletons_in_creation = self.singletons_currently_in_creation.lock().unwrap();
        singletons_in_creation.remove(name);
        drop(singletons_in_creation);

        Some(bean)
    }
}

