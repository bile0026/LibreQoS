use crate::TcHandle;
use lqos_config::Tunables;
use serde::{Deserialize, Serialize};

/// One or more `BusRequest` objects must be included in a `BusSession`
/// request. Each `BusRequest` represents a single request for action
/// or data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum BusRequest {
  /// A generic "is it alive?" test. Returns an `Ack`.
  Ping,

  /// Request total current throughput. Returns a
  /// `BusResponse::CurrentThroughput` value.
  GetCurrentThroughput,

  /// Retrieve the top N downloads by bandwidth use.
  GetTopNDownloaders {
    /// First row to retrieve (usually 0 unless you are paging)
    start: u32,
    /// Last row to retrieve (10 for top-10 starting at 0)
    end: u32,
  },

  /// Retrieves the TopN hosts with the worst RTT, sorted by RTT descending.
  GetWorstRtt {
    /// First row to retrieve (usually 0 unless you are paging)
    start: u32,
    /// Last row to retrieve (10 for top-10 starting at 0)
    end: u32,
  },

  /// Retrieves the TopN hosts with the best RTT, sorted by RTT descending.
  GetBestRtt {
    /// First row to retrieve (usually 0 unless you are paging)
    start: u32,
    /// Last row to retrieve (10 for top-10 starting at 0)
    end: u32,
  },

  /// Retrieves current byte counters for all hosts.
  GetHostCounter,

  /// Requests that the XDP back-end associate an IP address with a
  /// TC (traffic control) handle, and CPU. The "upload" flag indicates
  /// that this is a second channel applied to the SAME network interface,
  /// used for "on-a-stick" mode upload channels.
  MapIpToFlow {
    /// The IP address to map, as a string. It can be IPv4 or IPv6,
    /// and supports CIDR notation for subnets. "192.168.1.1",
    /// "192.168.1.0/24", are both valid.
    ip_address: String,

    /// The TC Handle to which the IP address should be mapped.
    tc_handle: TcHandle,

    /// The CPU on which the TC handle should be shaped.
    cpu: u32,

    /// If true, this is a *second* flow for the same IP range on
    /// the same NIC. Used for handling "on a stick" configurations.
    upload: bool,
  },

  /// Requests that the XDP program unmap an IP address/subnet from
  /// the traffic management system.
  DelIpFlow {
    /// The IP address to unmap. It can be an IPv4, IPv6 or CIDR
    /// subnet.
    ip_address: String,

    /// Should we delete a secondary mapping (for upload)?
    upload: bool,
  },

  /// Clear all XDP IP/TC/CPU mappings.
  ClearIpFlow,

  /// Retreieve list of all current IP/TC/CPU mappings.
  ListIpFlow,

  /// Simulate the previous version's `xdp_pping` command, returning
  /// RTT data for all mapped flows by TC handle.
  XdpPping,

  /// Divide current RTT data into histograms and return the data for
  /// rendering.
  RttHistogram,

  /// Cound the number of mapped and unmapped hosts detected by the
  /// system.
  HostCounts,

  /// Retrieve a list of all unmapped IPs that have been detected
  /// carrying traffic.
  AllUnknownIps,

  /// Reload the `LibreQoS.py` program and return details of the
  /// reload run.
  ReloadLibreQoS,

  /// Retrieve raw queue data for a given circuit ID.
  GetRawQueueData(String), // The string is the circuit ID

  /// Requests a real-time adjustment of the `lqosd` tuning settings
  UpdateLqosDTuning(u64, Tunables),

  /// Request that we start watching a circuit's queue
  WatchQueue(String),

  /// Request that the Rust side of things validate the CSV
  ValidateShapedDevicesCsv,

  /// Request details of part of the network tree
  GetNetworkMap {
    /// The parent of the map to retrieve
    parent: usize,
  },

  /// Retrieves the top N queues from the root level, and summarizes
  /// the others as "other"
  TopMapQueues(usize),

  /// Retrieve node names from network.json
  GetNodeNamesFromIds(Vec<usize>),

  /// Retrieve stats for all queues above a named circuit id
  GetFunnel {
    /// Circuit being analyzed, as the named circuit id
    target: String,
  },

  /// Obtain the lqosd statistics
  GetLqosStats,

  /// Tell me flow stats for a given IP address
  GetFlowStats(String),

  /// Tell Heimdall to hyper-focus on an IP address for a bit
  GatherPacketData(String),

  /// Give me a dump of the last 10 seconds of packet headers
  GetPacketHeaderDump(usize),

  /// Give me a libpcap format packet dump (shortened) of the last 10 seconds
  GetPcapDump(usize),

  /// If running on Equinix (the `equinix_test` feature is enabled),
  /// display a "run bandwidht test" link.
  #[cfg(feature = "equinix_tests")]
  RequestLqosEquinixTest,
}
