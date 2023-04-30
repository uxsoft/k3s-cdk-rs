use super::KubernetesComponent;

pub struct Ingress<'a> {
    app_name: &'a str,
    namespace: &'a str,
    ingress_name: &'a str,
    host: &'a str,
    service_name: &'a str,
    service_port: u16,
}

impl<'a> Ingress<'a> {
    pub fn new(
        app_name: &'a str,
        namespace: &'a str,
        ingress_name: &'a str,
        host: &'a str,
        service_name: &'a str,
        service_port: u16,
    ) -> Self {
        Self {
            app_name,
            namespace,
            ingress_name,
            host,
            service_name,
            service_port,
        }
    }
}

impl<'a> KubernetesComponent for Ingress<'a> {
    fn to_yaml(&self) -> String {
        let app_name = self.app_name;
        let namespace = self.namespace;
        let ingress_name = self.ingress_name;
        let host = self.host;
        let service_name = self.service_name;
        let service_port = self.service_port;

        format!("apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: {app_name}
  namespace: {namespace}
  labels:
    name: {app_name}
spec:
  rules:
  - host: {host}
    http:
      paths:
      - pathType: Prefix
        path: \"/\"
        backend:
          service:
            name: {service_name}
            port: 
              number: {service_port}"
        )
    }
}
