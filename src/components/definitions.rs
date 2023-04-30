pub trait KubernetesComponent {
    fn to_yaml(&self) -> String;
}

