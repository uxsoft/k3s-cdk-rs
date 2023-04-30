use super::KubernetesComponent;

pub struct Service<'a> {
    app_name: &'a str,
    namespace: &'a str,
    service_name: &'a str,
    service_port: u16,
    container_port: u16,
}

impl<'a> Service<'a> {
    pub fn new(
        app_name: &'a str,
        namespace: &'a str,
        service_name: &'a str,
        service_port: u16,
        container_port: u16,
    ) -> Self {
        Self {
            app_name,
            namespace,
            service_name,
            service_port,
            container_port,
        }
    }
}

impl<'a> KubernetesComponent for Service<'a> {
    fn to_yaml(&self) -> String {
        let app_name = self.app_name;
        let namespace = self.namespace;
        let service_name = self.service_name;
        let service_port = self.service_port;
        let container_port = self.container_port;

        format!(
            "apiVersion: v1
kind: Service
metadata:
  name: {service_name}
  namespace: {namespace}
spec:
  selector:
    app: {app_name}
  ports:
  - port: {service_port}
    targetPort: {container_port}"
        )
    }
}
