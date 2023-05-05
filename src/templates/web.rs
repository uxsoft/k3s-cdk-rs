use crate::components::*;
use crate::templates::*;

pub struct DockerWeb<'a> {
    name: &'a str,
    namespace: &'a str,
    host: &'a str,
}

impl<'a> DockerWeb<'a> {
    pub fn new(name: &'a str, namespace: &'a str, host: &'a str) -> Self {
        Self {
            name,
            namespace,
            host,
        }
    }
}

impl<'a> KubernetesTemplate for DockerWeb<'a> {
    fn to_yaml(&self) -> String {
        let service_name = format!("{}-service", self.name);
        let ingress_name = format!("{}-ingress", self.name);

        let service = Service::new(self.name, self.namespace, &service_name, 80, 80);

        let ingress = Ingress::new(
            self.name,
            self.namespace,
            &ingress_name,
            self.host,
            &service_name,
            0,
        );

        return vec![service.to_yaml(), ingress.to_yaml()].join("\n---\n");
    }
}
