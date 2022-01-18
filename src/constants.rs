pub const NAME: &str = "prometheus-netapp-exporter";
pub const VERSION: &str = "0.1.1-20220118";
pub const DEFAULT_INSECURE_SSL: bool = false;
pub const DEFAULT_TIMEOUT: u64 = 60;
pub const DEFAULT_PROMETHEUS_ADDRESS: &str = "localhost:9988";
const REPO_URL: &str = "https://ypbind.de/cgit/prometheus-netapp-exporter/";

pub fn generate_default_user_agent() -> String {
    format!("{}/{} ({})", NAME, VERSION, REPO_URL)
}
