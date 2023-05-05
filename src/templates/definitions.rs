pub trait KubernetesTemplate {
    fn to_yaml(&self) -> String;
    // fn build(&self) -> Vec<Box<dyn KubernetesComponent>>;
}