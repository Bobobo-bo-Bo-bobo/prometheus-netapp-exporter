pub const NAME: &str = "prometheus-netapp-exporter";
pub const VERSION: &str = "0.1.1-20220122";
pub const DEFAULT_INSECURE_SSL: bool = false;
pub const DEFAULT_TIMEOUT: u64 = 60;
pub const DEFAULT_PROMETHEUS_ADDRESS: &str = "localhost:9988";
const REPO_URL: &str = "https://ypbind.de/cgit/prometheus-netapp-exporter/";

pub fn generate_default_user_agent() -> String {
    format!("{}/{} ({})", NAME, VERSION, REPO_URL)
}

pub const ROOT_HTML: &str = "<html>\n<head><title>NetApp exporter</title></head>\n<body>\n<h1>NetApp exporter</h1>\n<p><a href=\"/metrics\">Metrics</a></p>\n</body>\n</html>\n";
pub const METRICS_PATH: &str = "/metrics";
pub const HTTP_CLIENT_TIMEOUT: u64 = 15;

pub const API_AGGREGATES: &str = "/api/storage/aggregates";

pub const NETAPP_STORAGE_AGGREGATES: &str = "/api/storage/aggregates";
pub const NETAPP_STORAGE_VOLUMES: &str = "/api/storage/volumes";

pub const TARGET_AGGREGATES: u64 = 0x0000000000000001;
pub const TARGET_VOLUMES: u64 = 0x0000000000000002;
