use common_multipart_rfc7578::client::multipart::BoundaryGenerator;
use rand::{distributions::Alphanumeric, rngs::SmallRng, Rng, SeedableRng};

pub struct TwilightBoundaryGenerator;

impl BoundaryGenerator for TwilightBoundaryGenerator {
    fn generate_boundary() -> String {
        let rng = SmallRng::from_entropy();
        let bytes = rng.sample_iter(&Alphanumeric);

        bytes.take(10).collect()
    }
}
