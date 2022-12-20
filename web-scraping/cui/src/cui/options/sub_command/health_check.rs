use crate::cui::options::Service;

#[derive(Debug, Clone, clap::Args, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HealthCheck {
    #[arg(short, long, value_enum, default_value_t=Service::Suumo, help = "対象サービスを指定する")]
    pub(crate) service: Service,
}
