use crate::server::concurrent::ConcurrentServer;

pub struct SequentialServer {
  server: ConcurrentServer,
}
