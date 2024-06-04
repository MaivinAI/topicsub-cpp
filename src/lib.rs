use crate::ffi::CameraInfo;
use crate::ffi::CompressedImage;
use crate::ffi::DeepviewDMABuf;
use crate::ffi::Detect;
use crate::ffi::FoxgloveImageAnnotations;
use crate::ffi::PointCloud2;
use crate::ffi::Response;
use cdr;
use ffi::ModelInfo;
use ffi::RadarCube;
use std::collections::HashMap;
use std::ptr::slice_from_raw_parts;
use std::str::FromStr;
use zenoh::{
    prelude::{r#async::*, sync::SyncResolve},
    subscriber::FlumeSubscriber,
};

#[cxx::bridge]
mod ffi {
    // Any shared structs, whose fields will be visible to both languages.
    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    struct Response {
        count: i64,
        data: Vec<u8>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct Time {
        pub sec: i32,
        pub nanosec: u32,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct Header {
        pub stamp: Time,
        pub frame_id: String,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct CompressedImage {
        pub header: Header,
        pub format: String,
        pub data: Vec<u8>,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct PointCloud2 {
        pub header: Header,
        pub height: u32,
        pub width: u32,
        pub fields: Vec<PointField>,
        pub is_bigendian: bool,
        pub point_step: u32,
        pub row_step: u32,
        pub data: Vec<u8>,
        pub is_dense: bool,
    }

    // TODO: Consts don't work right now. Maybe add them manually on the C++ side
    // mod point_field {
    //     pub const INT8: u8 = 1;
    //     pub const UINT8: u8 = 2;
    //     pub const INT16: u8 = 3;
    //     pub const UINT16: u8 = 4;
    //     pub const INT32: u8 = 5;
    //     pub const UINT32: u8 = 6;
    //     pub const FLOAT32: u8 = 7;
    //     pub const FLOAT64: u8 = 8;
    // }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct PointField {
        pub name: String,
        pub offset: u32,
        pub datatype: u8,
        pub count: u32,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct DeepviewDMABuf {
        pub header: Header,
        pub src_pid: u32,
        pub dma_fd: i32,
        pub width: u32,
        pub height: u32,
        pub stride: u32,
        pub fourcc: u32,
        pub length: u32,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct FoxgloveImageAnnotations {
        pub circles: Vec<FoxgloveCircleAnnotations>,
        pub points: Vec<FoxglovePointAnnotations>,
        pub texts: Vec<FoxgloveTextAnnotations>,
    }

    // TODO: Consts don't work right now. Maybe add them manually on the C++ side
    // pub mod point_annotation_type {
    //     pub const UNKNOWN: u8 = 0;
    //     // Individual points: 0, 1, 2, ...
    //     pub const POINTS: u8 = 1;
    //     // Closed polygon: 0-1, 1-2, ..., (n-1)-n, n-0
    //     pub const LINE_LOOP: u8 = 2;
    //     // Connected line segments: 0-1, 1-2, ..., (n-1)-n
    //     pub const LINE_STRIP: u8 = 3;
    //     // Individual line segments: 0-1, 2-3, 4-5, ...
    //     pub const LINE_LIST: u8 = 4;
    // }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct FoxgloveCircleAnnotations {
        pub timestamp: Time,
        pub position: FoxglovePoint2,
        pub diameter: f64,
        pub thickness: f64,
        pub fill_color: FoxgloveColor,
        pub outline_color: FoxgloveColor,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct FoxglovePointAnnotations {
        pub timestamp: Time,
        pub type_: u8,
        pub points: Vec<FoxglovePoint2>,
        pub outline_color: FoxgloveColor,
        pub outline_colors: Vec<FoxgloveColor>,
        pub fill_color: FoxgloveColor,
        pub thickness: f64,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct FoxgloveTextAnnotations {
        pub timestamp: Time,
        pub position: FoxglovePoint2,
        pub text: String,
        pub font_size: f64,
        pub text_color: FoxgloveColor,
        pub background_color: FoxgloveColor,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct FoxglovePoint2 {
        pub x: f64,
        pub y: f64,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct FoxgloveColor {
        pub r: f64,
        pub g: f64,
        pub b: f64,
        pub a: f64,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct CameraInfo {
        pub header: Header,
        pub height: u32,
        pub width: u32,
        pub distortion_model: String,
        pub d: Vec<f64>,
        pub k: [f64; 9],
        pub r: [f64; 9],
        pub p: [f64; 12],
        pub binning_x: u32,
        pub binning_y: u32,
        pub roi: RegionOfInterest,
    }
    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct RegionOfInterest {
        pub x_offset: u32,
        pub y_offset: u32,
        pub height: u32,
        pub width: u32,
        pub do_rectify: bool,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct Detect {
        pub header: Header,
        pub input_timestamp: Time,
        pub model_time: Time,
        pub output_time: Time,
        pub boxes: Vec<DetectBox2D>,
    }
    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct DetectBox2D {
        pub center_x: f32,
        pub center_y: f32,
        pub width: f32,
        pub height: f32,
        pub label: String,
        pub score: f32,
        pub distance: f32,
        pub speed: f32,
        pub track: DetectTrack,
    }
    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct DetectTrack {
        pub id: String,
        pub lifetime: i32,
        pub created: Time,
    }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct ModelInfo {
        pub header: Header,
        pub input_shape: Vec<u32>,
        pub input_type: u8,
        pub output_shape: Vec<u32>,
        pub output_type: u8,
        pub labels: Vec<String>,
        pub model_type: String,
        pub model_format: String,
        pub model_name: String,
    }

    // /// Dimensional labels are used to describe the radar cube layout. Not all cubes
    // /// include every label.  Undefined is used for dimensions not covered by this
    // /// list.  For example the Raivin radar cube is in the sequence, range,
    // /// rxchannel, and doppler dimensions.  These labels can be used so applications
    // /// can interpret the radar cube data genericly and portably between different
    // /// radar modules.
    // pub mod radar_cube_dimension {
    //     pub const UNDEFINED: u8 = 0;
    //     pub const RANGE: u8 = 1;
    //     pub const DOPPLER: u8 = 2;
    //     pub const AZIMUTH: u8 = 3;
    //     pub const ELEVATION: u8 = 4;
    //     pub const RXCHANNEL: u8 = 5;
    //     pub const SEQUENCE: u8 = 6;
    // }

    #[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
    pub struct RadarCube {
        /// Message header containing the timestamp and frame id.
        pub header: Header,
        /// The timestamp of the radar cube data as generated by the radar module.
        pub timestamp: u64,
        /// The layout of the radar cube.  The layout is a vector of dimension labels
        /// that describe the order of the dimensions in the cube.  The labels are
        /// defined in the radar_cube_dimension module.
        pub layout: Vec<u8>,
        /// The shape of the radar cube.
        pub shape: Vec<u16>,
        /// The scales vector is used to convert the cube bins to physical units.
        pub scales: Vec<f32>,
        /// The radar cube data.  The cube is represented as a 3D array of signed
        /// 16-bit integers.  If the cube is complex as indicated by the is_complex
        /// field, then the elements should be interpred as pairs of real and
        /// imaginary values.
        pub cube: Vec<i16>,
        /// The cube uses complex numbers if true.
        pub is_complex: bool,
    }

    extern "Rust" {
        // Functions implemented in Rust.
        unsafe fn deserialize_compressed_image(bytes: *const u8, len: usize) -> CompressedImage;
        unsafe fn deserialize_pointcloud2(bytes: *const u8, len: usize) -> PointCloud2;
        unsafe fn deserialize_dmabuf(bytes: *const u8, len: usize) -> DeepviewDMABuf;
        unsafe fn deserialize_image_annotations(
            bytes: *const u8,
            len: usize,
        ) -> FoxgloveImageAnnotations;
        unsafe fn deserialize_camera_info(bytes: *const u8, len: usize) -> CameraInfo;
        unsafe fn deserialize_detect(bytes: *const u8, len: usize) -> Detect;
        unsafe fn deserialize_model_info(bytes: *const u8, len: usize) -> ModelInfo;
        unsafe fn deserialize_radar_cube(bytes: *const u8, len: usize) -> RadarCube;
        // Zero or more opaque types which both languages can pass around
        // but only Rust can see the fields.
        type ZenohContext<'a>;
        // Functions implemented in Rust.
        unsafe fn create_zenoh_ctx<'a>() -> Box<ZenohContext<'a>>;
        unsafe fn add_subscriber_on_topic<'a>(
            ctx: &'a mut Box<ZenohContext<'a>>,
            topic: String,
        ) -> i32;

        // passing C function pointers into Rust is unimplemented, so we cannot implement a callback.
        // https://cxx.rs/binding/fn.html
        unsafe fn get_latest_data_from_subscriber<'a>(
            ctx: &mut Box<ZenohContext<'a>>,
            topic: String,
        ) -> Response;

        unsafe fn remove_subscriber_on_topic<'a>(ctx: &'a mut Box<ZenohContext<'a>>, topic: String);

    }
}

unsafe fn deserialize_compressed_image(bytes: *const u8, len: usize) -> CompressedImage {
    let slice = slice_from_raw_parts(bytes, len);
    cdr::deserialize::<CompressedImage>(unsafe { &*slice }).unwrap()
}

unsafe fn deserialize_pointcloud2(bytes: *const u8, len: usize) -> PointCloud2 {
    let slice = slice_from_raw_parts(bytes, len);
    cdr::deserialize::<PointCloud2>(unsafe { &*slice }).unwrap()
}

unsafe fn deserialize_dmabuf(bytes: *const u8, len: usize) -> DeepviewDMABuf {
    let slice = slice_from_raw_parts(bytes, len);
    cdr::deserialize::<DeepviewDMABuf>(unsafe { &*slice }).unwrap()
}

unsafe fn deserialize_image_annotations(bytes: *const u8, len: usize) -> FoxgloveImageAnnotations {
    let slice = slice_from_raw_parts(bytes, len);
    cdr::deserialize::<FoxgloveImageAnnotations>(unsafe { &*slice }).unwrap()
}

unsafe fn deserialize_camera_info(bytes: *const u8, len: usize) -> CameraInfo {
    let slice = slice_from_raw_parts(bytes, len);
    cdr::deserialize::<CameraInfo>(unsafe { &*slice }).unwrap()
}

unsafe fn deserialize_detect(bytes: *const u8, len: usize) -> Detect {
    let slice = slice_from_raw_parts(bytes, len);
    cdr::deserialize::<Detect>(unsafe { &*slice }).unwrap()
}

unsafe fn deserialize_model_info(bytes: *const u8, len: usize) -> ModelInfo {
    let slice = slice_from_raw_parts(bytes, len);
    cdr::deserialize::<ModelInfo>(unsafe { &*slice }).unwrap()
}

unsafe fn deserialize_radar_cube(bytes: *const u8, len: usize) -> RadarCube {
    let slice = slice_from_raw_parts(bytes, len);
    cdr::deserialize::<RadarCube>(unsafe { &*slice }).unwrap()
}

struct Sub<'a> {
    sub: FlumeSubscriber<'a>,
    count: i64,
}
struct ZenohContext<'a> {
    session: Session,
    subscribers: HashMap<String, Sub<'a>>,
}

unsafe fn create_zenoh_ctx<'a>() -> Box<ZenohContext<'a>> {
    let mut config = Config::default();

    let mode = WhatAmI::Client;
    config.set_mode(Some(mode)).unwrap();
    config.connect.endpoints = vec![EndPoint::from_str("tcp/127.0.0.1:7447").unwrap()];
    let _ = config.scouting.multicast.set_enabled(Some(false));
    let _ = config.scouting.gossip.set_enabled(Some(false));

    let session = zenoh::open(config).res_sync().unwrap();
    return Box::new(ZenohContext {
        session: session,
        subscribers: HashMap::new(),
    });
}

unsafe fn add_subscriber_on_topic<'a>(ctx: &'a mut Box<ZenohContext<'a>>, topic: String) -> i32 {
    let sub = match ctx.session.declare_subscriber(&topic).res_sync() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error declaring subscriber {topic}: {e}");
            return 1;
        }
    };

    ctx.subscribers.insert(topic, Sub { sub, count: 0 });
    return 0;
}

unsafe fn remove_subscriber_on_topic<'a>(ctx: &'a mut Box<ZenohContext<'a>>, topic: String) {
    ctx.subscribers.remove(&topic);
}

unsafe fn get_latest_data_from_subscriber<'a>(
    ctx: &mut Box<ZenohContext<'a>>,
    topic: String,
) -> Response {
    let mut resp = Response {
        count: -1,
        data: Vec::new(),
    };
    let sub = match ctx.subscribers.get_mut(&topic) {
        Some(s) => s,
        None => return resp,
    };
    let msgs = sub.sub.drain();
    sub.count += msgs.len() as i64;
    resp.count = sub.count;
    match msgs.last() {
        Some(s) => resp.data = s.payload.contiguous().to_vec(),
        None => {}
    }
    return resp;
}
