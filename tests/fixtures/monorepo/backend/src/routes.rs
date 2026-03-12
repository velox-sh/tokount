/// GET /health
pub fn health() -> &'static str {
	"ok"
}

/// GET /version
pub fn version() -> &'static str {
	env!("CARGO_PKG_VERSION")
}
