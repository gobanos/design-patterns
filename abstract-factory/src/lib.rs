trait AbstractFactory {
    type A: ProductA + 'static;
    type B: ProductB + 'static;

    fn create_product_a(&self) -> Self::A;
    fn create_product_b(&self) -> Self::B;
}

trait ProductA {}
trait ProductB {}

struct ConcreteFactory1 {}
impl AbstractFactory for ConcreteFactory1 {
    type A = ProductA1;
    type B = ProductB1;

    fn create_product_a(&self) -> Self::A {
        ProductA1 {}
    }

    fn create_product_b(&self) -> Self::B {
        ProductB1 {}
    }
}

struct ConcreteFactory2 {}
impl AbstractFactory for ConcreteFactory2 {
    type A = ProductA2;
    type B = ProductB2;

    fn create_product_a(&self) -> Self::A {
        ProductA2 {}
    }

    fn create_product_b(&self) -> Self::B {
        ProductB2 {}
    }
}

struct ProductA1 {}
impl ProductA for ProductA1 {}

struct ProductB1 {}
impl ProductB for ProductB1 {}

struct ProductA2 {}
impl ProductA for ProductA2 {}

struct ProductB2 {}
impl ProductB for ProductB2 {}

struct Client {
    product_a: Box<ProductA>,
    product_b: Box<ProductB>,
}

impl Client {
    fn with_factory<F: AbstractFactory>(factory: &F) -> Client {
        Client {
            product_a: Box::new(factory.create_product_a()),
            product_b: Box::new(factory.create_product_b()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concrete_factory_1() {
        let factory = ConcreteFactory1 {};
        let client = Client::with_factory(&factory);
    }

    #[test]
    fn concrete_factory_2() {
        let factory = ConcreteFactory2 {};
        let client = Client::with_factory(&factory);
    }
}
