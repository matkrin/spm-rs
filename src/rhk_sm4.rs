use anyhow::Result;
use std::{fs::read, io::Cursor};

use crate::utils::Bytereading;

#[derive(Debug)]
enum RhkDataType {
    DataImage,
    DataLine,
    DataXyData,
    DataAnnotatedLine,
    DataText,
    DataAnnotatedText,
    DataSequential, /* Only in RHKPageIndex */
    DataMovie,      /* New in R9, cannot import it anyway. */
    Unkwown,
}

impl RhkDataType {
    fn from_num(num: u32) -> Self {
        match num {
            0 => Self::DataImage,
            1 => Self::DataLine,
            2 => Self::DataXyData,
            3 => Self::DataAnnotatedLine,
            4 => Self::DataText,
            5 => Self::DataAnnotatedText,
            6 => Self::DataSequential,
            7 => Self::DataMovie,
            _ => Self::Unkwown,
        }
    }
}

#[derive(Debug)]
enum RhkObjectType {
    Undefined,           //= 0,
    PageIndexHeader,     //= 1,
    PageIndexArray,      //= 2,
    PageHeader,          //= 3,
    PageData,            //= 4,
    ImageDriftHeader,    //= 5,
    ImageDrift,          //= 6,
    SpecDriftHeader,     //= 7,
    SpecDriftData,       //= 8,
    ColorInfo,           //= 9,
    StringData,          //= 10,
    TipTrackHeader,      //= 11,
    TipTrackData,        //= 12,
    Prm,                 //= 13,
    Thumbnail,           //= 14,
    PrmHeader,           //= 15,
    ThumbnailHeader,     //= 16,
    ApiInfo,             //= 17,
    HistoryInfo,         //= 18,
    PiezoSensitivity,    //= 19,
    FrequencySweepData,  //= 20,
    ScanProcessorInfo,   //= 21,
    PllInfo,             //= 22,
    Ch1DriveInfo,        //= 23,
    Ch2DriveInfo,        //= 24,
    Lockin0Info,         //= 25,
    Lockin1Info,         //= 26,
    ZpiInfo,             //= 27,
    KpiInfo,             //= 28,
    AuxPiInfo,           //= 29,
    LowpassFilterR0Info, //= 30,
    LowpassFilterR1Info, //= 31,
    _FileHeader,          //= -42,
    _PageIndex,           //= -43,
    Unkwown,
}

impl RhkObjectType {
    fn from_num(num: u32) -> Self {
        match num {
            0 => Self::Undefined,            //= 0,
            1 => Self::PageIndexHeader,      //= 1,
            2 => Self::PageIndexArray,       //= 2,
            3 => Self::PageHeader,           //= 3,
            4 => Self::PageData,             //= 4,
            5 => Self::ImageDriftHeader,     //= 5,
            6 => Self::ImageDrift,           //= 6,
            7 => Self::SpecDriftHeader,      //= 7,
            8 => Self::SpecDriftData,        //= 8,
            9 => Self::ColorInfo,            //= 9,
            10 => Self::StringData,          //= 10,
            11 => Self::TipTrackHeader,      //= 11,
            12 => Self::TipTrackData,        //= 12,
            13 => Self::Prm,                 //= 13,
            14 => Self::Thumbnail,           //= 14,
            15 => Self::PrmHeader,           //= 15,
            16 => Self::ThumbnailHeader,     //= 16,
            17 => Self::ApiInfo,             //= 17,
            18 => Self::HistoryInfo,         //= 18,
            19 => Self::PiezoSensitivity,    //= 19,
            20 => Self::FrequencySweepData,  //= 20,
            21 => Self::ScanProcessorInfo,   //= 21,
            22 => Self::PllInfo,             //= 22,
            23 => Self::Ch1DriveInfo,        //= 23,
            24 => Self::Ch2DriveInfo,        //= 24,
            25 => Self::Lockin0Info,         //= 25,
            26 => Self::Lockin1Info,         //= 26,
            27 => Self::ZpiInfo,             //= 27,
            28 => Self::KpiInfo,             //= 28,
            29 => Self::AuxPiInfo,           //= 29,
            30 => Self::LowpassFilterR0Info, //= 30,
            31 => Self::LowpassFilterR1Info, //= 31,
            // -42 => Self::FileHeader,          //= -42,
            // -43 => Self::PageIndex,           //= -43,
            _ => Self::Unkwown,
        }
    }
}

#[derive(Debug)]
enum RhkSourceType {
    SourceRaw,        //= 0,
    SourceProcessed,  //= 1,
    SourceCalculated, //= 2,
    SourceImported,   //= 3,
    Unknown,
}

impl RhkSourceType {
    fn from_num(num: u32) -> Self {
        match num {
            1 => Self::SourceRaw,        //= 0,
            2 => Self::SourceProcessed,  //= 1,
            3 => Self::SourceCalculated, //= 2,
            4 => Self::SourceImported,   //= 3,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum RhkImageType {
    Normal,         //= 0,
    Autocorrelated, //= 1,
    Unknown,
}

impl RhkImageType {
    fn from_num(num: u32) -> Self {
        match num {
            1 => Self::Normal,         //= 0,
            2 => Self::Autocorrelated, //= 1,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum RhkPageType {
    Undefined,                 //= 0,
    Topographic,               //= 1,
    Current,                   //= 2,
    Aux,                       //= 3,
    Force,                     //= 4,
    Signal,                    //= 5,
    Fft,                       //= 6,
    NoisePowerSpectrum,        //= 7,
    LineTest,                  //= 8,
    Oscilloscope,              //= 9,
    IVSpectra,                 //= 10,
    IV4x4,                     //= 11,
    IV8x8,                     //= 12,
    IV16x16,                   //= 13,
    IV32x32,                   //= 14,
    IVCenter,                  //= 15,
    InteractiveSpectra,        //= 16,
    Autocorreclation,          //= 17,
    IZSpectra,                 //= 18,
    Gain4Topography,           //= 19,
    Gain8Topography,           //= 20,
    Gain4Current,              //= 21,
    Gain8Current,              //= 22,
    IV64x64,                   //= 23,
    AutocorrelationSpectrum,   //= 24,
    Counter,                   //= 25,
    MultichannelAnalyser,      //= 26,
    Afm100,                    //= 27,
    Cits,                      //= 28,
    Gpib,                      //= 29,
    VideoChannel,              //= 30,
    ImageOutSpectra,           //= 31,
    IDatalog,                  //= 32,
    IEcset,                    //= 33,
    IEcdata,                   //= 34,
    IDspAd,                    //= 35,
    DiscreteSpectroscopyPp,    //= 36,
    ImageDiscreteSpectroscopy, //= 37,
    RampSpectroscopyRp,        //= 38,
    DiscreteSpectroscopyRp,    //= 39,
    Unknown,
}

impl RhkPageType {
    fn from_num(num: u32) -> Self {
        match num {
            0 => Self::Undefined,                  //= 0,
            1 => Self::Topographic,                //= 1,
            2 => Self::Current,                    //= 2,
            3 => Self::Aux,                        //= 3,
            4 => Self::Force,                      //= 4,
            5 => Self::Signal,                     //= 5,
            6 => Self::Fft,                        //= 6,
            7 => Self::NoisePowerSpectrum,         //= 7,
            8 => Self::LineTest,                   //= 8,
            9 => Self::Oscilloscope,               //= 9,
            10 => Self::IVSpectra,                 //= 10,
            11 => Self::IV4x4,                     //= 11,
            12 => Self::IV8x8,                     //= 12,
            13 => Self::IV16x16,                   //= 13,
            14 => Self::IV32x32,                   //= 14,
            15 => Self::IVCenter,                  //= 15,
            16 => Self::InteractiveSpectra,        //= 16,
            17 => Self::Autocorreclation,          //= 17,
            18 => Self::IZSpectra,                 //= 18,
            19 => Self::Gain4Topography,           //= 19,
            20 => Self::Gain8Topography,           //= 20,
            21 => Self::Gain4Current,              //= 21,
            22 => Self::Gain8Current,              //= 22,
            23 => Self::IV64x64,                   //= 23,
            24 => Self::AutocorrelationSpectrum,   //= 24,
            25 => Self::Counter,                   //= 25,
            26 => Self::MultichannelAnalyser,      //= 26,
            27 => Self::Afm100,                    //= 27,
            28 => Self::Cits,                      //= 28,
            29 => Self::Gpib,                      //= 29,
            30 => Self::VideoChannel,              //= 30,
            31 => Self::ImageOutSpectra,           //= 31,
            32 => Self::IDatalog,                  //= 32,
            33 => Self::IEcset,                    //= 33,
            34 => Self::IEcdata,                   //= 34,
            35 => Self::IDspAd,                    //= 35,
            36 => Self::DiscreteSpectroscopyPp,    //= 36,
            37 => Self::ImageDiscreteSpectroscopy, //= 37,
            38 => Self::RampSpectroscopyRp,        //= 38,
            39 => Self::DiscreteSpectroscopyRp,    //= 39,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum RhkLineType {
    NotALine,                     //= 0,
    Histogram,                    //= 1,
    CrossSection,                 //= 2,
    LineTest,                     //= 3,
    Oscilloscope,                 //= 4,
    NoisePowerSpectrum,           //= 6,
    IvSpectrum,                  //= 7,
    IzSpectrum,                  //= 8,
    ImageXAverage,                //= 9,
    ImageYAverage,                //= 10,
    NoiseAutocorrelationSpectrum, //= 11,
    MultichannelAnalyserData,     //= 12,
    RenormalizedIv,              //= 13,
    ImageHistogramSpectra,        //= 14,
    ImageCrossSection,            //= 15,
    ImageAverage,                 //= 16,
    ImageCrossSectionG,           //= 17,
    ImageOutSpectra,              //= 18,
    DatalogSpectrum,              //= 19,
    Gxy,                          //= 20,
    Electrochemistry,             //= 21,
    DiscreteSpectroscopy,         //= 22,
    DscopeDatalogging,            //= 23,
    TimeSpectroscopy,             //= 24,
    ZoomFft,                      //= 25,
    FrequencySweep,               //= 26,
    PhaseRotate,                  //= 27,
    FiberSweep,                   //= 28,
    Unknown,
}

impl RhkLineType {
    fn from_num(num: u32) -> Self {
        match num {
            1 => Self::NotALine,                      //= 0,
            2 => Self::Histogram,                     //= 1,
            3 => Self::CrossSection,                  //= 2,
            4 => Self::LineTest,                      //= 3,
            5 => Self::Oscilloscope,                  //= 4,
            6 => Self::NoisePowerSpectrum,            //= 6,
            7 => Self::IvSpectrum,                   //= 7,
            8 => Self::IzSpectrum,                   //= 8,
            9 => Self::ImageXAverage,                 //= 9,
            10 => Self::ImageYAverage,                //= 10,
            11 => Self::NoiseAutocorrelationSpectrum, //= 11,
            12 => Self::MultichannelAnalyserData,     //= 12,
            13 => Self::RenormalizedIv,              //= 13,
            14 => Self::ImageHistogramSpectra,        //= 14,
            15 => Self::ImageCrossSection,            //= 15,
            16 => Self::ImageAverage,                 //= 16,
            17 => Self::ImageCrossSectionG,           //= 17,
            18 => Self::ImageOutSpectra,              //= 18,
            19 => Self::DatalogSpectrum,              //= 19,
            20 => Self::Gxy,                          //= 20,
            21 => Self::Electrochemistry,             //= 21,
            22 => Self::DiscreteSpectroscopy,         //= 22,
            23 => Self::DscopeDatalogging,            //= 23,
            24 => Self::TimeSpectroscopy,             //= 24,
            25 => Self::ZoomFft,                      //= 25,
            26 => Self::FrequencySweep,               //= 26,
            27 => Self::PhaseRotate,                  //= 27,
            28 => Self::FiberSweep,                   //= 28,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum RhkScanType {
    ScanRight, //= 0,
    ScanLeft,  //= 1,
    ScanUp,    //= 2,
    ScanDown,  //= 3,
    Unknown,
}

impl RhkScanType {
    fn from_num(num: u32) -> Self {
        match num {
            1 => Self::ScanRight,
            2 => Self::ScanLeft,
            3 => Self::ScanUp,
            4 => Self::ScanDown,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum RhkStringType {
    Label,
    SystemText,
    SessionText,
    UserText,
    Path,
    Date,
    Time,
    XUnits,
    YUnits,
    ZUnits,
    XLnits,
    YLnits,
    StatusChannelText,
    CompletedLineCount,
    OversamplingCount,
    SlicedVoltage,
    PllProStatus,
    NString,
    Unknown,
}

impl RhkStringType {
    fn from_num(num: u32) -> Self {
        match num {
            1 => Self::Label,
            2 => Self::SystemText,
            3 => Self::SessionText,
            4 => Self::UserText,
            5 => Self::Path,
            6 => Self::Date,
            7 => Self::Time,
            8 => Self::XUnits,
            9 => Self::YUnits,
            10 => Self::ZUnits,
            11 => Self::XLnits,
            12 => Self::YLnits,
            13 => Self::StatusChannelText,
            14 => Self::CompletedLineCount,
            15 => Self::OversamplingCount,
            16 => Self::SlicedVoltage,
            17 => Self::PllProStatus,
            18 => Self::NString,
            _ => Self::Unknown,
        }
    }
}

#[derive(Debug)]
enum RhkPiezoStringType {
    TubeXUnit,
    TubeYUnit,
    TubeZUnit,
    TubeZoffetUnit,
    ScanXUnit,
    ScanYUnit,
    ScanZUnit,
    ActuatorUnit,
    TubeCalibration,
    ScanCalibration,
    AcutuatorCalibration,
    NString,
}

#[derive(Debug)]
enum RhkDriftOptionType {
    Disabled,      //= 0,
    Each_Spectra,  //= 1,
    Each_Location, //= 2
    Unknown,
}

impl RhkDriftOptionType {
    fn from_num(num: u32) -> Self {
        match num {
            1 => Self::Disabled,      //= 0,
            2 => Self::Each_Spectra,  //= 1,
            3 => Self::Each_Location, //= 2
            _ => Self::Unknown,
        }
    }
}

// #[derive(Debug)]
// struct RhkSpecDriftHeader{
//     start_time: u64,
//      drift_opt: RhkDriftOptionType,
//     nstrings: u32,
//     strings: Vec<String>,
// }

// #[derive(Debug)]
// struct RhkPiezoSensitivity {
//     tube_x: f64,
//     tube_y: f64,
//     tube_z: f64,
//     tube_z_offset: f64,
//     scan_x: f64,
//     scan_y: f64,
//     scan_z: f64,
//     actuator: f64,
//     string_count: u32,
//     strings: Vec<String>, //[RHK_PIEZO_NSTRINGS];
// }

// #[derive(Debug)]
// struct RhkSpecInfo {
//     ftime: f64,
//     x_coord: f64,
//     y_coord: f64,
//     dx: f64,
//     dy: f64,
//     cumulative_dx: f64,
//     cumulative_dy: f64,
// }

// #[derive(Debug)]
// struct RhkObject {
//     object_type: RhkObjectType,
//     offset: u32,
//     size: u32,
// }

// #[derive(Debug)]
// struct RhkPageIndexHeader {
//     page_count: u32,
//     object_count: u32,
//     // reserved_1: u32,
//     // reserved_2: u32,
//     objects: Vec<RhkObject>,
// }

// #[derive(Debug)]
// struct RhkPage {
//      field_size: u32,
//      string_count: u32,
//     page_type: RhkPageType,
//      data_sub_source: u32,
//     line_type: RhkLineType,
//      x_coord: i32,
//      y_coord: i32,
//      x_size: u32,
//      y_size: u32,
//     image_type: RhkImageType,
//     scan_dir: RhkScanType,
//      group_id: u32,
//      data_size: u32,
//      min_z_value: u32,
//      max_z_value: u32,
//      x_scale: f64,
//      y_scale: f64,
//      z_scale: f64,
//      xy_scale: f64,
//      x_offset: f64,
//      y_offset: f64,
//      z_offset: f64,
//      period: f64,
//      bias: f64,
//      current: f64,
//      angle: f64,
//      color_info_count: u32,
//      grid_x_size: u32,
//      grid_y_size: u32,
//      object_count: u32,
//      // reserved: u32,16],
//     // const guchar *data,
//     strings: Vec<String>,
//     objects: Vec<RhkObject>,
//     drift_header: RhkDriftHeader,
//     spec_info: RhkSpecInfo,
//     piezo_sensitivity: RhkPiezoSensitivity,
// }

// #[derive(Debug)]
// struct RhkPageIndex {
//     guchar id[GUID_SIZE];
//     data_type :RhkDataType,
//     source: RhkSourceType,
//     object_count: u32,
//     minor_version: u32,
//     objects: Vec<RhkObject>,
//     page: RhkPage,
// }

// #[derive(Debug)]
// struct RhkFile {
//     page_count: u32,
//     object_count: u32,
//     object_field_size: u32,
//     reserved1: u32,
//     reserved2: u32,
//     objects: Vec<RhkObject>,
//     page_index_header: RhkPageIndexHeader,
//     page_indices: RhkPageIndex,
// }

///////////////////////////////

#[derive(Debug)]
struct Sm4Header {
    size: u16,
    signature: String,
    page_count: u32,
    object_list_count: u32,
    object_field_size: u32,
    object_list: Vec<Sm4Object>,
}

#[derive(Debug)]
struct Sm4Object {
    obj_type: RhkObjectType,
    offset: u32,
    size: u32,
}

#[derive(Debug)]
struct PageIndexHeader {
    offset: u32,
    page_count: u32,
    object_count: u32,
    // objects: Sm4Object,
}

#[derive(Debug)]
struct Sm4Page {
    page_id: u16,
    page_data_type: RhkDataType,
    page_source_type: RhkSourceType,
    object_list_count: u32,
    minor_version: u32,
    object_list: Vec<Sm4Object>,
}

#[derive(Debug)]
enum Sm4PageHeader {
    Sequential(Sm4PageHeaderSequential),
    Default(Sm4PageHeaderDefault),
}

#[derive(Debug)]
struct Sm4PageHeaderSequential {
    data_type: u32,
    data_length: u32,
    param_count: u32,
    object_list_count: u32,
    data_info_size: u32,
    data_info_string_count: u32,
    object_list: Vec<Sm4Object>,
}

#[derive(Debug)]
struct Sm4PageHeaderDefault {
    string_count: u16,
    page_type: RhkPageType,
    // Channel: e.g. Topograhic, Current,...
    data_sub_source: u32,
    line_type: RhkLineType,
    x_corner: u32,
    y_corner: u32,
    // xres
    x_size: u32,
    // yres
    y_size: u32,
    image_type: RhkImageType,
    scan_type: RhkScanType,
    group_id: u32,
    page_data_size: u32,
    min_z_value: u32,
    max_z_value: u32,
    // x_scale * x_size gives physical dimensions
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    xy_scale: f32,
    // offsets
    x_offset: f32,
    y_offset: f32,
    z_offset: f32,
    period: f32,
    // bias
    bias: f32,
    // current
    current: f32,
    // rotation
    angle: f32,
    color_info_count: u32,
    grid_x_size: u32,
    grid_y_size: u32,
    object_list_count: u32,
    _32_bit_data_flag: u8,
    object_list: Vec<Sm4Object>,
}

#[derive(Debug)]
pub struct Sm4Image {
    pub current: f64,
    pub bias: f64,
    pub xsize: f64,
    pub ysize: f64,
    pub xres: u32,
    pub yres: u32,
    pub rotation: f64,
    pub raster_time: f64,
    pub xoffset: f64,
    pub yoffset: f64,
    pub data: Vec<f64>,
}

pub fn read_rhk_sm4(filename: &str) -> Result<Vec<Sm4Image>> {
    let bytes = read(filename)?;
    let _file_len = bytes.len();
    let mut cursor = Cursor::new(bytes.as_slice());

    let mut header = read_header(&mut cursor);

    for _ in 0..header.object_list_count {
        header.object_list.push(read_sm4_object(&mut cursor))
    }

    let page_index_header = get_page_index_header(&mut cursor, &header.object_list)?;

    let mut page_index_header_list = Vec::with_capacity(page_index_header.object_count as usize);
    for _ in 0..page_index_header.object_count {
        page_index_header_list.push(read_sm4_object(&mut cursor))
    }

    let page_index_array_offset = get_offset_page_index_array(&page_index_header_list)?;
    cursor.set_position(page_index_array_offset as u64);

    let mut pages = Vec::with_capacity(page_index_header.page_count as usize);
    for _ in 0..page_index_header.page_count {
        let mut page = read_sm4_page(&mut cursor);

        for _ in 0..page.object_list_count {
            page.object_list.push(read_sm4_object(&mut cursor));
        }

        pages.push(page);
    }

    let mut page_objects = Vec::with_capacity(pages.len());
    let mut images = Vec::new();
    for page in &pages {
        let mut page_header = read_page_header(&mut cursor, &page)?;
        match page_header {
            Sm4PageHeader::Sequential(ref mut ph) => {
                ph.object_list.push(read_sm4_object(&mut cursor));

                let mut sequential_param_gain = Vec::with_capacity(ph.param_count as usize);
                let mut sequential_param_label = Vec::with_capacity(ph.param_count as usize);
                let mut sequential_param_unit = Vec::with_capacity(ph.param_count as usize);
                for _ in 0..ph.param_count {
                    sequential_param_gain.push(cursor.read_f32_le());
                    sequential_param_label.push(read_sm4_string(&mut cursor));
                    sequential_param_unit.push(read_sm4_string(&mut cursor));
                }
            }
            Sm4PageHeader::Default(ref mut ph) => ph.object_list.push(read_sm4_object(&mut cursor)),
        }
        let mut tiptrack_info_count = 0;
        let mut read_objects = Vec::with_capacity(page.object_list.len());
        for obj in &page.object_list {
            if obj.offset != 0 && obj.size != 0 {
                let read_obj =
                    read_object_content(obj, &page_header, &mut cursor, &mut tiptrack_info_count)?;
                match (&read_obj, &page_header) {
                    (ReadType::PageData(data), Sm4PageHeader::Default(ph)) => {
                        if let RhkPageType::Topographic = ph.page_type {
                            images.push(Sm4Image {
                                current: ph.current as f64,
                                bias: ph.bias as f64,
                                xsize: (ph.x_scale as f64 * ph.x_size as f64).abs(),
                                ysize: (ph.y_size as f64 * ph.y_scale as f64).abs(),
                                xres: ph.x_size,
                                yres: ph.y_size,
                                rotation: ph.angle as f64,
                                raster_time: ph.period as f64,
                                xoffset: ph.x_offset as f64,
                                yoffset: ph.y_offset as f64,
                                data: data.clone(),
                            });
                        }
                    }
                    _ => {}
                };
                read_objects.push(read_obj);
            }
        }
        page_objects.push(read_objects)
    }

    Ok(images)
}

fn read_object_content(
    obj: &Sm4Object,
    page_header: &Sm4PageHeader,
    cursor: &mut Cursor<&[u8]>,
    tiptrack_info_count: &mut u32,
) -> Result<ReadType> {
    let read_obj = match obj.obj_type {
        RhkObjectType::PageData => {
            if let Sm4PageHeader::Default(ph) = page_header {
                read_page_data(cursor, obj.offset, obj.size, ph.z_scale, ph.z_offset)
            } else {
                ReadType::Unknown
            }
        }
        RhkObjectType::ImageDriftHeader => read_image_drift_header(cursor, obj.offset),
        RhkObjectType::ImageDrift => read_image_drift(cursor, obj.offset),
        RhkObjectType::SpecDriftHeader => read_spec_drift_header(cursor, obj.offset),
        RhkObjectType::SpecDriftData => {
            if let Sm4PageHeader::Default(ph) = page_header {
                read_spec_drift_data(cursor, obj.offset, ph.y_size)
            } else {
                ReadType::Unknown
            }
        }
        RhkObjectType::ColorInfo => ReadType::Unknown,
        RhkObjectType::StringData => {
            if let Sm4PageHeader::Default(ph) = page_header {
                read_string_data(cursor, obj.offset, ph.string_count)
            } else {
                ReadType::Unknown
            }
        }
        RhkObjectType::TipTrackHeader => {
            let tiptrack_header = read_tip_track_header(cursor, obj.offset);
            if let ReadType::TipTrackHeader(tth) = &tiptrack_header {
                *tiptrack_info_count = tth.tiptrack_tiptrack_info_count;
            }
            tiptrack_header
        }
        RhkObjectType::TipTrackData => {
            read_tip_track_data(cursor, obj.offset, *tiptrack_info_count)
        }
        RhkObjectType::Prm => ReadType::Unknown,
        RhkObjectType::PrmHeader => {
            if let Sm4PageHeader::Default(ph) = page_header {
                read_prm_header(cursor, obj.offset, &ph.object_list)?
            } else {
                ReadType::Unknown
            }
        }
        RhkObjectType::ApiInfo => read_api_info(cursor, obj.offset),
        RhkObjectType::HistoryInfo => read_history_info(cursor, obj.offset),
        RhkObjectType::PiezoSensitivity => read_piezo_sensitivity(cursor, obj.offset),
        RhkObjectType::FrequencySweepData => read_frequency_sweep_data(cursor, obj.offset),
        RhkObjectType::ScanProcessorInfo => read_scan_processor_info(cursor, obj.offset),
        RhkObjectType::PllInfo => read_pll_info(cursor, obj.offset),

        RhkObjectType::Ch1DriveInfo => read_channel_drive_info(cursor, obj.offset),
        RhkObjectType::Ch2DriveInfo => read_channel_drive_info(cursor, obj.offset),

        RhkObjectType::Lockin0Info => read_lockin_info(cursor, obj.offset),
        RhkObjectType::Lockin1Info => read_lockin_info(cursor, obj.offset),

        RhkObjectType::ZpiInfo => read_pi_controller_info(cursor, obj.offset),
        RhkObjectType::KpiInfo => read_pi_controller_info(cursor, obj.offset),
        RhkObjectType::AuxPiInfo => read_pi_controller_info(cursor, obj.offset),

        RhkObjectType::LowpassFilterR0Info => read_lowpass_filter_info(cursor, obj.offset),
        RhkObjectType::LowpassFilterR1Info => read_lowpass_filter_info(cursor, obj.offset),
        _ => ReadType::Unknown,
    };
    Ok(read_obj)
}

fn read_sm4_string(cursor: &mut Cursor<&[u8]>) -> String {
    let length = cursor.read_u16_le();
    cursor.read_string(length as usize)
}

fn get_object_type_name(object_type_id: u32) -> String {
    let name = match object_type_id {
        0 => "RHK_OBJECT_UNDEFINED",
        1 => "RHK_OBJECT_PAGE_INDEX_HEADER",
        2 => "RHK_OBJECT_PAGE_INDEX_ARRAY",
        3 => "RHK_OBJECT_PAGE_HEADER",
        4 => "RHK_OBJECT_PAGE_DATA",
        5 => "RHK_OBJECT_IMAGE_DRIFT_HEADER",
        6 => "RHK_OBJECT_IMAGE_DRIFT",
        7 => "RHK_OBJECT_SPEC_DRIFT_HEADER",
        8 => "RHK_OBJECT_SPEC_DRIFT_DATA",
        9 => "RHK_OBJECT_COLOR_INFO",
        10 => "RHK_OBJECT_STRING_DATA",
        11 => "RHK_OBJECT_TIP_TRACK_HEADER",
        12 => "RHK_OBJECT_TIP_TRACK_DATA",
        13 => "RHK_OBJECT_PRM",
        14 => "RHK_OBJECT_THUMBNAIL",
        15 => "RHK_OBJECT_PRM_HEADER",
        16 => "RHK_OBJECT_THUMBNAIL_HEADER",
        17 => "RHK_OBJECT_API_INFO",
        18 => "RHK_OBJECT_HISTORY_INFO",
        19 => "RHK_OBJECT_PIEZO_SENSITIVITY",
        20 => "RHK_OBJECT_FREQUENCY_SWEEP_DATA",
        21 => "RHK_OBJECT_SCAN_PROCESSOR_INFO",
        22 => "RHK_OBJECT_PLL_INFO",
        23 => "RHK_OBJECT_CH1_DRIVE_INFO",
        24 => "RHK_OBJECT_CH2_DRIVE_INFO",
        25 => "RHK_OBJECT_LOCKIN0_INFO",
        26 => "RHK_OBJECT_LOCKIN1_INFO",
        27 => "RHK_OBJECT_ZPI_INFO",
        28 => "RHK_OBJECT_KPI_INFO",
        29 => "RHK_OBJECT_AUX_PI_INFO",
        30 => "RHK_OBJECT_LOWPASS_FILTER0_INFO",
        31 => "RHK_OBJECT_LOWPASS_FILTER1_INFO",
        _ => "RHK_OBJECT_UNKNOWN",
    };
    name.to_string()
}

fn get_page_data_type_name(page_data_type: u32) -> String {
    let page_data_type_name = match page_data_type {
        0 => "RHK_DATA_IMAGE",
        1 => "RHK_DATA_LINE",
        2 => "RHK_DATA_XY_DATA",
        3 => "RHK_DATA_ANNOTATED_LINE",
        4 => "RHK_DATA_TEXT",
        5 => "RHK_DATA_ANNOTATED_TEXT",
        6 => "RHK_DATA_SEQUENTIAL",
        7 => "RHK_DATA_MOVIE",
        _ => "RHK_DATA_UNKOWN",
    };
    page_data_type_name.to_string()
}

fn get_page_source_type_name(page_source_type: u32) -> String {
    let page_source_type_name = match page_source_type {
        0 => "RHK_SOURCE_RAW",
        1 => "RHK_SOURCE_PROCESSED",
        2 => "RHK_SOURCE_CALCULATED",
        3 => "RHK_SOURCE_IMPORTED",
        _ => "RHK_SOURCE_UNKNOWN",
    };
    page_source_type_name.to_string()
}

fn get_page_type_name(page_type: u32) -> String {
    let page_type_name = match page_type {
        0 => "RHK_PAGE_UNDEFINED",
        1 => "RHK_PAGE_TOPOGRAPHIC",
        2 => "RHK_PAGE_CURRENT",
        3 => "RHK_PAGE_AUX",
        4 => "RHK_PAGE_FORCE",
        5 => "RHK_PAGE_SIGNAL",
        6 => "RHK_PAGE_FFT_TRANSFORM",
        7 => "RHK_PAGE_NOISE_POWER_SPECTRUM",
        8 => "RHK_PAGE_LINE_TEST",
        9 => "RHK_PAGE_OSCILLOSCOPE",
        10 => "RHK_PAGE_IV_SPECTRA",
        11 => "RHK_PAGE_IV_4x4",
        12 => "RHK_PAGE_IV_8x8",
        13 => "RHK_PAGE_IV_16x16",
        14 => "RHK_PAGE_IV_32x32",
        15 => "RHK_PAGE_IV_CENTER",
        16 => "RHK_PAGE_INTERACTIVE_SPECTRA",
        17 => "RHK_PAGE_AUTOCORRELATION",
        18 => "RHK_PAGE_IZ_SPECTRA",
        19 => "RHK_PAGE_4_GAIN_TOPOGRAPHY",
        20 => "RHK_PAGE_8_GAIN_TOPOGRAPHY",
        21 => "RHK_PAGE_4_GAIN_CURRENT",
        22 => "RHK_PAGE_8_GAIN_CURRENT",
        23 => "RHK_PAGE_IV_64x64",
        24 => "RHK_PAGE_AUTOCORRELATION_SPECTRUM",
        25 => "RHK_PAGE_COUNTER",
        26 => "RHK_PAGE_MULTICHANNEL_ANALYSER",
        27 => "RHK_PAGE_AFM_100",
        28 => "RHK_PAGE_CITS",
        29 => "RHK_PAGE_GPIB",
        30 => "RHK_PAGE_VIDEO_CHANNEL",
        31 => "RHK_PAGE_IMAGE_OUT_SPECTRA",
        32 => "RHK_PAGE_I_DATALOG",
        33 => "RHK_PAGE_I_ECSET",
        34 => "RHK_PAGE_I_ECDATA",
        35 => "RHK_PAGE_I_DSP_AD",
        36 => "RHK_PAGE_DISCRETE_SPECTROSCOPY_PP",
        37 => "RHK_PAGE_IMAGE_DISCRETE_SPECTROSCOPY",
        38 => "RHK_PAGE_RAMP_SPECTROSCOPY_RP",
        39 => "RHK_PAGE_DISCRETE_SPECTROSCOPY_RP",
        _ => "RHK_PAGE_TYPE_UNKWOWN",
    };
    page_type_name.to_string()
}

fn get_line_type_name(line_type: u32) -> String {
    let line_type_name = match line_type {
        0 => "RHK_LINE_NOT_A_LINE",
        1 => "RHK_LINE_HISTOGRAM",
        2 => "RHK_LINE_CROSS_SECTION",
        3 => "RHK_LINE_LINE_TEST",
        4 => "RHK_LINE_OSCILLOSCOPE",
        5 => "RHK_LINE_RESERVED",
        6 => "RHK_LINE_NOISE_POWER_SPECTRUM",
        7 => "RHK_LINE_IV_SPECTRUM",
        8 => "RHK_LINE_IZ_SPECTRUM",
        9 => "RHK_LINE_IMAGE_X_AVERAGE",
        10 => "RHK_LINE_IMAGE_Y_AVERAGE",
        11 => "RHK_LINE_NOISE_AUTOCORRELATION_SPECTRUM",
        12 => "RHK_LINE_MULTICHANNEL_ANALYSER_DATA",
        13 => "RHK_LINE_RENORMALIZED_IV",
        14 => "RHK_LINE_IMAGE_HISTOGRAM_SPECTRA",
        15 => "RHK_LINE_IMAGE_CROSS_SECTION",
        16 => "RHK_LINE_IMAGE_AVERAGE",
        17 => "RHK_LINE_IMAGE_CROSS_SECTION_G",
        18 => "RHK_LINE_IMAGE_OUT_SPECTRA",
        19 => "RHK_LINE_DATALOG_SPECTRUM",
        20 => "RHK_LINE_GXY",
        21 => "RHK_LINE_ELECTROCHEMISTRY",
        22 => "RHK_LINE_DISCRETE_SPECTROSCOPY",
        23 => "RHK_LINE_DATA_LOGGER",
        24 => "RHK_LINE_TIME_SPECTROSCOPY",
        25 => "RHK_LINE_ZOOM_FFT",
        26 => "RHK_LINE_FREQUENCY_SWEEP",
        27 => "RHK_LINE_PHASE_ROTATE",
        28 => "RHK_LINE_FIBER_SWEEP",
        _ => "RHK_LINE_TYPE_UKNWOWN",
    };
    line_type_name.to_string()
}

fn get_image_type_name(image_type: u32) -> String {
    let image_type_name = match image_type {
        0 => "RHK_IMAGE_NORMAL",
        1 => "RHK_IMAGE_AUTOCORRELATED",
        _ => "RHK_IMAGE_TYPE_UNKNOWN",
    };
    image_type_name.to_string()
}

fn get_scan_type_name(scan_type: u32) -> String {
    let scan_type_name = match scan_type {
        0 => "RHK_SCAN_RIGHT",
        1 => "RHK_SCAN_LEFT",
        2 => "RHK_SCAN_UP",
        3 => "RHK_SCAN_DOWN",
        _ => "RHK_SCAN_TYPE_UNKWOWN",
    };
    scan_type_name.to_string()
}

fn get_drift_option_type_name(drift_option_type: u32) -> String {
    let imagedrift_drift_option_type_name = match drift_option_type {
        0 => "RHK_DRIFT_DISABLED",
        1 => "RHK_DRIFT_EACH_SPECTRA",
        2 => "RHK_DRIFT_EACH_LOCATION",
        _ => "RHK_DRIFT_UNKNOWN",
    };
    imagedrift_drift_option_type_name.to_string()
}

fn get_page_index_header(
    cursor: &mut Cursor<&[u8]>,
    object_list: &Vec<Sm4Object>,
) -> Result<PageIndexHeader> {
    let offset = get_offset_page_index_header(object_list)?;
    cursor.set_position(offset as u64);
    let page_count = cursor.read_u32_le();
    let object_list_count = cursor.read_u32_le();
    let _reserved_1 = cursor.read_u32_le();
    let _reserved_2 = cursor.read_u32_le();
    Ok(PageIndexHeader {
        offset,
        page_count,
        object_count: object_list_count,
    })
}

fn get_offset_page_index_header(object_list: &Vec<Sm4Object>) -> Result<u32> {
    for obj in object_list {
        if let RhkObjectType::PageIndexHeader = obj.obj_type {
            return Ok(obj.offset);
        }
    }
    Err(anyhow::anyhow!("No page index header"))
}

fn get_offset_page_index_array(object_list: &Vec<Sm4Object>) -> Result<u32> {
    for obj in object_list {
        if let RhkObjectType::PageIndexArray = obj.obj_type {
            return Ok(obj.offset);
        }
    }
    Err(anyhow::anyhow!("No page index array"))
}

fn read_header(cursor: &mut Cursor<&[u8]>) -> Sm4Header {
    let size = cursor.read_u16_le();
    let signature = cursor.read_string(36);
    let page_count = cursor.read_u32_le();
    let object_list_count = cursor.read_u32_le();
    let object_field_size = cursor.read_u32_le();

    let _reserved_1 = cursor.read_u32_le();
    let _reserved_2 = cursor.read_u32_le();

    Sm4Header {
        size,
        signature,
        page_count,
        object_list_count,
        object_field_size,
        object_list: Vec::with_capacity(object_list_count as usize),
    }
}

fn read_sm4_object(cursor: &mut Cursor<&[u8]>) -> Sm4Object {
    let object_type_id = RhkObjectType::from_num(cursor.read_u32_le());
    let offset = cursor.read_u32_le();
    let size = cursor.read_u32_le();

    Sm4Object {
        obj_type: object_type_id,
        offset,
        size,
    }
}

fn read_sm4_page(cursor: &mut Cursor<&[u8]>) -> Sm4Page {
    let page_id = cursor.read_u16_le();
    cursor.skip(14);
    let page_data_type = RhkDataType::from_num(cursor.read_u32_le());

    let page_source_type = RhkSourceType::from_num(cursor.read_u32_le());

    let object_list_count = cursor.read_u32_le();

    let minor_version = cursor.read_u32_le();

    Sm4Page {
        page_id,
        page_data_type,
        page_source_type,
        minor_version,
        object_list_count,
        object_list: Vec::with_capacity(object_list_count as usize),
    }
}

fn read_page_header(cursor: &mut Cursor<&[u8]>, page: &Sm4Page) -> Result<Sm4PageHeader> {
    let offset = get_offset_object_page_header(&page.object_list)?;
    cursor.set_position(offset as u64);
    // Sequential data type
    if let RhkDataType::DataSequential = page.page_data_type {
        return Ok(Sm4PageHeader::Sequential(read_sequential_type(
            cursor, page,
        )));
    }
    Ok(Sm4PageHeader::Default(read_default_type(cursor, page)))
}

fn get_offset_object_page_header(object_list: &Vec<Sm4Object>) -> Result<u32> {
    for obj in object_list {
        if let RhkObjectType::PageHeader = obj.obj_type {
            return Ok(obj.offset);
        }
    }
    Err(anyhow::anyhow!("No page header"))
}

fn read_sequential_type(cursor: &mut Cursor<&[u8]>, page: &Sm4Page) -> Sm4PageHeaderSequential {
    let data_type = cursor.read_u32_le();
    let data_length = cursor.read_u32_le();
    let param_count = cursor.read_u32_le();

    let object_list_count = cursor.read_u32_le();

    let data_info_size = cursor.read_u32_le();
    let data_info_string_count = cursor.read_u32_le();
    Sm4PageHeaderSequential {
        data_type,
        data_length,
        param_count,
        object_list_count,
        data_info_size,
        data_info_string_count,
        object_list: Vec::with_capacity(object_list_count as usize),
    }
}

fn read_default_type(cursor: &mut Cursor<&[u8]>, page: &Sm4Page) -> Sm4PageHeaderDefault {
    _ = cursor.read_u16_le();
    let string_count = cursor.read_u16_le();
    let page_type = RhkPageType::from_num(cursor.read_u32_le());
    let data_sub_source = cursor.read_u32_le();

    let line_type = RhkLineType::from_num(cursor.read_u32_le());

    let x_corner = cursor.read_u32_le();
    let y_corner = cursor.read_u32_le();
    // xres
    let x_size = cursor.read_u32_le();
    let y_size = cursor.read_u32_le();

    let image_type = RhkImageType::from_num(cursor.read_u32_le());

    let scan_type = RhkScanType::from_num(cursor.read_u32_le());

    let group_id = cursor.read_u32_le();
    let page_data_size = cursor.read_u32_le();

    let min_z_value = cursor.read_u32_le();
    let max_z_value = cursor.read_u32_le();

    let x_scale = cursor.read_f32_le();
    let y_scale = cursor.read_f32_le();
    let z_scale = cursor.read_f32_le();
    let xy_scale = cursor.read_f32_le();
    let x_offset = cursor.read_f32_le();
    let y_offset = cursor.read_f32_le();
    let z_offset = cursor.read_f32_le();
    let period = cursor.read_f32_le();
    let bias = cursor.read_f32_le();
    let current = cursor.read_f32_le();
    let angle = cursor.read_f32_le();

    let color_info_count = cursor.read_u32_le();
    let grid_x_size = cursor.read_u32_le();
    let grid_y_size = cursor.read_u32_le();

    let object_list_count = cursor.read_u32_le();
    let _32_bit_data_flag = cursor.read_u8_le();

    //reserved
    cursor.skip(63);

    Sm4PageHeaderDefault {
        string_count,
        page_type,
        data_sub_source,
        line_type,
        x_corner,
        y_corner,
        x_size,
        y_size,
        image_type,
        scan_type,
        group_id,
        page_data_size,
        min_z_value,
        max_z_value,
        x_scale,
        y_scale,
        z_scale,
        xy_scale,
        x_offset,
        y_offset,
        z_offset,
        period,
        bias,
        current,
        angle,
        color_info_count,
        grid_x_size,
        grid_y_size,
        object_list_count,
        _32_bit_data_flag,
        object_list: Vec::with_capacity(object_list_count as usize),
    }
}

fn read_page_data(
    cursor: &mut Cursor<&[u8]>,
    offset: u32,
    size: u32,
    z_scale: f32,
    z_offset: f32,
) -> ReadType {
    cursor.set_position(offset as u64);
    let len = size / 4;
    let mut page_data = Vec::with_capacity(len as usize);
    for _ in 0..len {
        page_data.push(cursor.read_i32_le() as f64 * (z_scale as f64) + (z_offset as f64));
    }
    ReadType::PageData(page_data)
}

#[derive(Debug)]
enum ReadType {
    PageData(Vec<f64>),
    ImageDriftHeader(ImageDriftHeader),
    ImageDriftData(ImageDriftData),
    SpecDriftHeader(SpecDriftHeader),
    SpecDriftData(SpecDriftData),
    StringData(StringData),
    TipTrackHeader(TipTrackHeader),
    TipTrackData(TipTrackData),
    Prm(Prm),
    ApiInfo(ApiInfo),
    PiezoSensitivity(PiezoSensitivity),
    FrequencySweepData(FrequencySweepData),
    ScanprocessorInfo(ScanProcessorInfo),
    PllInfo(PllInfo),
    ChannelDriveInfo(ChannelDriveInfo),
    LockinInfo(LockinInfo),
    PiControllerInfo(PiControllerInfo),
    LowpassFilterInfo(LowpassFilterInfo),
    HistoryInfo,
    Unknown,
}

#[derive(Debug)]
struct ImageDriftHeader {
    imagedrift_filetime: u64,
    imagedrift_drift_option_type: RhkDriftOptionType,
}

#[derive(Debug)]
struct ImageDriftData {
    imagedrift_time: u32,
    imagedrift_dx: u32,
    imagedrift_dy: u32,
    imagedrift_cumulative_x: u32,
    imagedrift_cumulative_y: u32,
    imagedrift_vector_x: u32,
    imagedrift_vector_y: u32,
}

#[derive(Debug)]
struct SpecDriftHeader {
    specdrift_filetime: u64,
    specdrift_drift_option_type: u32,
    specdrift_drift_option_type_name: String,
    specdrift_channel: String,
}

#[derive(Debug)]
struct SpecDriftData {
    specdrift_time: Vec<f32>,
    specdrift_x_coord: Vec<f32>,
    specdrift_y_coord: Vec<f32>,
    specdrift_dx: Vec<f32>,
    specdrift_dy: Vec<f32>,
    specdrift_cumulative_x: Vec<f32>,
    specdrift_cumulative_y: Vec<f32>,
}

fn read_image_drift_header(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    // unix epoch
    let imagedrift_filetime = cursor.read_u64_le();
    let imagedrift_drift_option_type = RhkDriftOptionType::from_num(cursor.read_u32_le());
    ReadType::ImageDriftHeader(ImageDriftHeader {
        imagedrift_filetime,
        imagedrift_drift_option_type,
    })
}

fn read_image_drift(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let imagedrift_time = cursor.read_u32_le();
    let imagedrift_dx = cursor.read_u32_le();
    let imagedrift_dy = cursor.read_u32_le();
    let imagedrift_cumulative_x = cursor.read_u32_le();
    let imagedrift_cumulative_y = cursor.read_u32_le();
    let imagedrift_vector_x = cursor.read_u32_le();
    let imagedrift_vector_y = cursor.read_u32_le();
    ReadType::ImageDriftData(ImageDriftData {
        imagedrift_time,
        imagedrift_dx,
        imagedrift_dy,
        imagedrift_cumulative_x,
        imagedrift_cumulative_y,
        imagedrift_vector_x,
        imagedrift_vector_y,
    })
}

fn read_spec_drift_header(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    // unix epoch
    let specdrift_filetime = cursor.read_u64_le();
    let specdrift_drift_option_type = cursor.read_u32_le();
    let specdrift_drift_option_type_name = get_drift_option_type_name(specdrift_drift_option_type);
    _ = cursor.read_u32_le();
    let specdrift_channel = read_sm4_string(cursor);

    ReadType::SpecDriftHeader(SpecDriftHeader {
        specdrift_filetime,
        specdrift_drift_option_type,
        specdrift_drift_option_type_name,
        specdrift_channel,
    })
}

fn read_spec_drift_data(cursor: &mut Cursor<&[u8]>, offset: u32, y_size: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let mut specdrift_time = Vec::with_capacity(y_size as usize);
    let mut specdrift_x_coord = Vec::with_capacity(y_size as usize);
    let mut specdrift_y_coord = Vec::with_capacity(y_size as usize);
    let mut specdrift_dx = Vec::with_capacity(y_size as usize);
    let mut specdrift_dy = Vec::with_capacity(y_size as usize);
    let mut specdrift_cumulative_x = Vec::with_capacity(y_size as usize);
    let mut specdrift_cumulative_y = Vec::with_capacity(y_size as usize);

    for _ in 0..y_size {
        specdrift_time.push(cursor.read_f32_le());
        specdrift_x_coord.push(cursor.read_f32_le());
        specdrift_y_coord.push(cursor.read_f32_le());
        specdrift_dx.push(cursor.read_f32_le());
        specdrift_dy.push(cursor.read_f32_le());
        specdrift_cumulative_x.push(cursor.read_f32_le());
        specdrift_cumulative_y.push(cursor.read_f32_le());
    }
    ReadType::SpecDriftData(SpecDriftData {
        specdrift_time,
        specdrift_x_coord,
        specdrift_y_coord,
        specdrift_dx,
        specdrift_dy,
        specdrift_cumulative_x,
        specdrift_cumulative_y,
    })
}

#[derive(Debug)]
struct StringData {
    label: String,
    system_text: String,
    session_text: String,
    user_text: String,
    filename: String,
    date: String,
    time: String,
    x_units: String,
    y_units: String,
    z_units: String,
    x_label: String,
    y_label: String,
    status_channel_text: String,
    completed_line_count: String,
    oversampling_count: String,
    sliced_voltage: String,
    pll_pro_status: String,
    setpoint_unit: String,
    channel_list: String,
}

fn read_string_data(cursor: &mut Cursor<&[u8]>, offset: u32, string_count: u16) -> ReadType {
    cursor.set_position(offset as u64);
    let label = read_sm4_string(cursor);
    let system_text = read_sm4_string(cursor);
    let session_text = read_sm4_string(cursor);
    let user_text = read_sm4_string(cursor);
    let filename = read_sm4_string(cursor);
    let date = read_sm4_string(cursor);
    let time = read_sm4_string(cursor);
    let x_units = read_sm4_string(cursor);
    let y_units = read_sm4_string(cursor);
    let z_units = read_sm4_string(cursor);
    let x_label = read_sm4_string(cursor);
    let y_label = read_sm4_string(cursor);
    let status_channel_text = read_sm4_string(cursor);
    let completed_line_count = read_sm4_string(cursor);
    let oversampling_count = read_sm4_string(cursor);
    let sliced_voltage = read_sm4_string(cursor);
    let pll_pro_status = read_sm4_string(cursor);
    let setpoint_unit = read_sm4_string(cursor);
    let channel_list = read_sm4_string(cursor);
    ReadType::StringData(StringData {
        label,
        system_text,
        session_text,
        user_text,
        filename,
        date,
        time,
        x_units,
        y_units,
        z_units,
        x_label,
        y_label,
        status_channel_text,
        completed_line_count,
        oversampling_count,
        sliced_voltage,
        pll_pro_status,
        setpoint_unit,
        channel_list,
    })
}

#[derive(Debug)]
struct TipTrackHeader {
    tiptrack_filetime: u64,
    tiptrack_feature_height: f32,
    tiptrack_feature_width: f32,
    tiptrack_time_constant: f32,
    tiptrack_cycle_rate: f32,
    tiptrack_phase_lag: f32,
    tiptrack_tiptrack_info_count: u32,
    tiptrack_channel: String,
}

fn read_tip_track_header(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    // epoch time
    let tiptrack_filetime = cursor.read_u64_le();
    let tiptrack_feature_height = cursor.read_f32_le();
    let tiptrack_feature_width = cursor.read_f32_le();

    let tiptrack_time_constant = cursor.read_f32_le();
    let tiptrack_cycle_rate = cursor.read_f32_le();
    let tiptrack_phase_lag = cursor.read_f32_le();
    _ = cursor.read_u32_le();
    let tiptrack_tiptrack_info_count = cursor.read_u32_le();
    let tiptrack_channel = read_sm4_string(cursor);
    ReadType::TipTrackHeader(TipTrackHeader {
        tiptrack_filetime,
        tiptrack_feature_height,
        tiptrack_feature_width,
        tiptrack_time_constant,
        tiptrack_cycle_rate,
        tiptrack_phase_lag,
        tiptrack_tiptrack_info_count,
        tiptrack_channel,
    })
}

#[derive(Debug)]
struct TipTrackData {
    tiptrack_cumulative_time: Vec<f32>,
    tiptrack_time: Vec<f32>,
    tiptrack_dx: Vec<f32>,
    tiptrack_dy: Vec<f32>,
}

fn read_tip_track_data(
    cursor: &mut Cursor<&[u8]>,
    offset: u32,
    tiptrack_info_count: u32,
) -> ReadType {
    cursor.set_position(offset as u64);
    let mut tiptrack_cumulative_time = Vec::with_capacity(tiptrack_info_count as usize);
    let mut tiptrack_time = Vec::with_capacity(tiptrack_info_count as usize);
    let mut tiptrack_dx = Vec::with_capacity(tiptrack_info_count as usize);
    let mut tiptrack_dy = Vec::with_capacity(tiptrack_info_count as usize);
    for _ in 0..tiptrack_info_count {
        tiptrack_cumulative_time.push(cursor.read_f32_le());
        tiptrack_time.push(cursor.read_f32_le());
        tiptrack_dx.push(cursor.read_f32_le());
        tiptrack_dy.push(cursor.read_f32_le());
    }
    ReadType::TipTrackData(TipTrackData {
        tiptrack_cumulative_time,
        tiptrack_time,
        tiptrack_dx,
        tiptrack_dy,
    })
}

#[derive(Debug)]
struct Prm {
    prm_compression_flag: u32,
    prm_data_size: u32,
    prm_compression_size: u32,
    prm_data: Vec<u32>,
}

fn read_prm_header(
    cursor: &mut Cursor<&[u8]>,
    offset: u32,
    object_list: &Vec<Sm4Object>,
) -> Result<ReadType> {
    cursor.set_position(offset as u64);
    let prm_compression_flag = cursor.read_u32_le();
    let prm_data_size = cursor.read_u32_le();
    let prm_compression_size = cursor.read_u32_le();

    let prm_data_offset = get_offset_object_prm(object_list)?;
    let prm_data = read_prm_data(cursor, prm_data_offset, prm_data_size, prm_compression_flag)?;
    Ok(ReadType::Prm(Prm {
        prm_compression_flag,
        prm_data_size,
        prm_compression_size,
        prm_data,
    }))
}

fn get_offset_object_prm(object_list: &Vec<Sm4Object>) -> Result<u32> {
    for obj in object_list {
        if let RhkObjectType::Prm = obj.obj_type {
            return Ok(obj.offset);
        }
    }
    Err(anyhow::anyhow!("No page index array"))
}

fn read_prm_data(
    cursor: &mut Cursor<&[u8]>,
    offset: u32,
    prm_data_size: u32,
    prm_compression_flag: u32,
) -> Result<Vec<u32>> {
    cursor.set_position(offset as u64);
    let mut prm_data = Vec::with_capacity(prm_data_size as usize);
    if prm_compression_flag == 0 {
        for _ in 0..prm_data_size {
            prm_data.push(cursor.read_u32_le())
        }
    } else {
        return Err(anyhow::anyhow!("Compressed Data not supported"));
    }
    Ok(prm_data)
}

#[derive(Debug)]
struct ApiInfo {
    voltage_high: f32,
    voltage_low: f32,
    gain: f32,
    api_offset: f32,
    ramp_type: u32,
    step: u32,
    image_count: u32,
    dac: u32,
    mux: u32,
    bias: u32,
}

fn read_api_info(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let voltage_high = cursor.read_f32_le();
    let voltage_low = cursor.read_f32_le();
    let gain = cursor.read_f32_le();
    let api_offset = cursor.read_f32_le();

    let ramp_mode = cursor.read_u32_le();
    let ramp_type = cursor.read_u32_le();
    let step = cursor.read_u32_le();
    let image_count = cursor.read_u32_le();
    let dac = cursor.read_u32_le();
    let mux = cursor.read_u32_le();
    let bias = cursor.read_u32_le();

    _ = cursor.read_u32_le();
    let units = read_sm4_string(cursor);

    ReadType::ApiInfo(ApiInfo {
        voltage_high,
        voltage_low,
        gain,
        api_offset,
        ramp_type,
        step,
        image_count,
        dac,
        mux,
        bias,
    })
}

fn read_history_info(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    _ = cursor.read_u32_le();
    _ = read_sm4_string(cursor);
    _ = read_sm4_string(cursor);
    ReadType::HistoryInfo
}

#[derive(Debug)]
struct PiezoSensitivity {
    tube_x: f64,
    tube_y: f64,
    tube_z: f64,
    tube_z_offset: f64,
    scan_x: f64,
    scan_y: f64,
    scan_z: f64,
    actuator: f64,
    tube_z_unit: String,
    tube_z_unit_offset: String,
    scan_x_unit: String,
    scan_y_unit: String,
    scan_z_unit: String,
    actuator_unit: String,
    tube_calibration: String,
    scan_calibration: String,
    actuator_calibration: String,
}

fn read_piezo_sensitivity(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let tube_x = cursor.read_f64_le();
    let tube_y = cursor.read_f64_le();
    let tube_z = cursor.read_f64_le();
    let tube_z_offset = cursor.read_f64_le();
    let scan_x = cursor.read_f64_le();
    let scan_y = cursor.read_f64_le();
    let scan_z = cursor.read_f64_le();
    let actuator = cursor.read_f64_le();

    _ = cursor.read_u32_le();

    let tube_x_unit = read_sm4_string(cursor);
    let tube_y_unit = read_sm4_string(cursor);
    let tube_z_unit = read_sm4_string(cursor);
    let tube_z_unit_offset = read_sm4_string(cursor);
    let scan_x_unit = read_sm4_string(cursor);
    let scan_y_unit = read_sm4_string(cursor);
    let scan_z_unit = read_sm4_string(cursor);
    let actuator_unit = read_sm4_string(cursor);
    let tube_calibration = read_sm4_string(cursor);
    let scan_calibration = read_sm4_string(cursor);
    let actuator_calibration = read_sm4_string(cursor);
    ReadType::PiezoSensitivity(PiezoSensitivity {
        tube_x,
        tube_y,
        tube_z,
        tube_z_offset,
        scan_x,
        scan_y,
        scan_z,
        actuator,
        tube_z_unit,
        tube_z_unit_offset,
        scan_x_unit,
        scan_y_unit,
        scan_z_unit,
        actuator_unit,
        tube_calibration,
        scan_calibration,
        actuator_calibration,
    })
}

#[derive(Debug)]
struct FrequencySweepData {
    psd_total_signal: f64,
    peak_frequency: f64,
    peak_amplitude: f64,
    drive_aplitude: f64,
    signal_to_drive_ratio: f64,
    q_factor: f64,
    total_signal_unit: String,
    peak_frequency_unit: String,
    peak_amplitude_unit: String,
    drive_amplitude_unit: String,
    signal_to_drive_ratio_unit: String,
    q_factor_unit: String,
}

fn read_frequency_sweep_data(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let psd_total_signal = cursor.read_f64_le();
    let peak_frequency = cursor.read_f64_le();
    let peak_amplitude = cursor.read_f64_le();
    let drive_aplitude = cursor.read_f64_le();
    let signal_to_drive_ratio = cursor.read_f64_le();
    let q_factor = cursor.read_f64_le();
    _ = cursor.read_u32_le();
    let total_signal_unit = read_sm4_string(cursor);
    let peak_frequency_unit = read_sm4_string(cursor);
    let peak_amplitude_unit = read_sm4_string(cursor);
    let drive_amplitude_unit = read_sm4_string(cursor);
    let signal_to_drive_ratio_unit = read_sm4_string(cursor);
    let q_factor_unit = read_sm4_string(cursor);
    ReadType::FrequencySweepData(FrequencySweepData {
        psd_total_signal,
        peak_frequency,
        peak_amplitude,
        drive_aplitude,
        signal_to_drive_ratio,
        q_factor,
        total_signal_unit,
        peak_frequency_unit,
        peak_amplitude_unit,
        drive_amplitude_unit,
        signal_to_drive_ratio_unit,
        q_factor_unit,
    })
}

#[derive(Debug)]
struct ScanProcessorInfo {
    x_slope_compensation: f64,
    y_slope_compensation: f64,
    x_slope_compensation_unit: String,
    y_slope_compensation_unit: String,
}

fn read_scan_processor_info(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let x_slope_compensation = cursor.read_f64_le();
    let y_slope_compensation = cursor.read_f64_le();
    _ = cursor.read_u32_le();
    let x_slope_compensation_unit = read_sm4_string(cursor);
    let y_slope_compensation_unit = read_sm4_string(cursor);
    ReadType::ScanprocessorInfo(ScanProcessorInfo {
        x_slope_compensation,
        y_slope_compensation,
        x_slope_compensation_unit,
        y_slope_compensation_unit,
    })
}

#[derive(Debug)]
struct PllInfo {
    amplitude_control: u32,
    drive_amplitude: f64,
    drive_ref_frequency: f64,
    lockin_freq_offset: f64,
    lockin_harmonic_factor: f64,
    lockin_phase_offset: f64,
    pi_gain: f64,
    pi_int_cutoff_frequency: f64,
    pi_lower_bound: f64,
    pi_upper_bound: f64,
    diss_pi_gain: f64,
    diss_pi_int_cutoff_frequency: f64,
    diss_pi_lower_bound: f64,
    diss_pi_upper_bound: f64,

    lockin_filter_cutoff_frequency: String,

    drive_amplitude_unit: String,
    drive_ref_frequency_unit: String,
    lockin_freq_offset_unit: String,
    lockin_harmonic_factor_unit: String,
    lockin_phase_offset_unit: String,
    pi_gain_unit: String,
    pi_int_cutoff_frequency_unit: String,
    pi_lower_bound_unit: String,
    pi_upper_bound_unit: String,
    diss_pi_gain_unit: String,
    diss_pi_int_cutoff_frequency_unit: String,
    diss_pi_lower_bound_unit: String,
    diss_pi_upper_bound_unit: String,
}

fn read_pll_info(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let amplitude_control = cursor.read_u32_le();
    let drive_amplitude = cursor.read_f64_le();
    let drive_ref_frequency = cursor.read_f64_le();
    let lockin_freq_offset = cursor.read_f64_le();
    let lockin_harmonic_factor = cursor.read_f64_le();
    let lockin_phase_offset = cursor.read_f64_le();
    let pi_gain = cursor.read_f64_le();
    let pi_int_cutoff_frequency = cursor.read_f64_le();
    let pi_lower_bound = cursor.read_f64_le();
    let pi_upper_bound = cursor.read_f64_le();
    let diss_pi_gain = cursor.read_f64_le();
    let diss_pi_int_cutoff_frequency = cursor.read_f64_le();
    let diss_pi_lower_bound = cursor.read_f64_le();
    let diss_pi_upper_bound = cursor.read_f64_le();

    let lockin_filter_cutoff_frequency = read_sm4_string(cursor);

    let drive_amplitude_unit = read_sm4_string(cursor);
    let drive_ref_frequency_unit = read_sm4_string(cursor);
    let lockin_freq_offset_unit = read_sm4_string(cursor);
    let lockin_harmonic_factor_unit = read_sm4_string(cursor);
    let lockin_phase_offset_unit = read_sm4_string(cursor);
    let pi_gain_unit = read_sm4_string(cursor);
    let pi_int_cutoff_frequency_unit = read_sm4_string(cursor);
    let pi_lower_bound_unit = read_sm4_string(cursor);
    let pi_upper_bound_unit = read_sm4_string(cursor);
    let diss_pi_gain_unit = read_sm4_string(cursor);
    let diss_pi_int_cutoff_frequency_unit = read_sm4_string(cursor);
    let diss_pi_lower_bound_unit = read_sm4_string(cursor);
    let diss_pi_upper_bound_unit = read_sm4_string(cursor);
    ReadType::PllInfo(PllInfo {
        amplitude_control,
        drive_amplitude,
        drive_ref_frequency,
        lockin_freq_offset,
        lockin_harmonic_factor,
        lockin_phase_offset,
        pi_gain,
        pi_int_cutoff_frequency,
        pi_lower_bound,
        pi_upper_bound,
        diss_pi_gain,
        diss_pi_int_cutoff_frequency,
        diss_pi_lower_bound,
        diss_pi_upper_bound,
        lockin_filter_cutoff_frequency,
        drive_amplitude_unit,
        drive_ref_frequency_unit,
        lockin_freq_offset_unit,
        lockin_harmonic_factor_unit,
        lockin_phase_offset_unit,
        pi_gain_unit,
        pi_int_cutoff_frequency_unit,
        pi_lower_bound_unit,
        pi_upper_bound_unit,
        diss_pi_gain_unit,
        diss_pi_int_cutoff_frequency_unit,
        diss_pi_lower_bound_unit,
        diss_pi_upper_bound_unit,
    })
}

#[derive(Debug)]
struct ChannelDriveInfo {
    master_osciallator: u32,
    amplitude: f64,
    frequency: f64,
    phase_offset: f64,
    harmonic_factor: f64,
    amplitude_unit: String,
    frequency_unit: String,
    phase_offset_unit: String,
    harmonic_factor_unit: String,
}

fn read_channel_drive_info(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    _ = cursor.read_u32_le();
    let master_osciallator = cursor.read_u32_le();

    let amplitude = cursor.read_f64_le();
    let frequency = cursor.read_f64_le();
    let phase_offset = cursor.read_f64_le();
    let harmonic_factor = cursor.read_f64_le();

    let amplitude_unit = read_sm4_string(cursor);
    let frequency_unit = read_sm4_string(cursor);
    let phase_offset_unit = read_sm4_string(cursor);
    let harmonic_factor_unit = read_sm4_string(cursor);
    ReadType::ChannelDriveInfo(ChannelDriveInfo {
        master_osciallator,
        amplitude,
        frequency,
        phase_offset,
        harmonic_factor,
        amplitude_unit,
        frequency_unit,
        phase_offset_unit,
        harmonic_factor_unit,
    })
}

#[derive(Debug)]
struct LockinInfo {
    num_strings: u32,
    non_master_oscillator: u32,
    frequency: f64,
    harmonic_factor: f64,
    phase_offset: f64,
    // these might be not included
    filter_cutoff_frequency: String,
    frequency_unit: String,
    phase_unit: String,
}

fn read_lockin_info(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let num_strings = cursor.read_u32_le();

    let non_master_oscillator = cursor.read_u32_le();
    let frequency = cursor.read_f64_le();
    let harmonic_factor = cursor.read_f64_le();
    let phase_offset = cursor.read_f64_le();
    // these might be not included
    let filter_cutoff_frequency = read_sm4_string(cursor);
    let frequency_unit = read_sm4_string(cursor);
    let phase_unit = read_sm4_string(cursor);
    ReadType::LockinInfo(LockinInfo {
        num_strings,
        non_master_oscillator,
        frequency,
        harmonic_factor,
        phase_offset,
        filter_cutoff_frequency,
        frequency_unit,
        phase_unit,
    })
}

#[derive(Debug)]
struct PiControllerInfo {
    setpoint: f64,
    proportional_gain: f64,
    integral_gain: f64,
    lower_bound: f64,
    upper_bound: f64,
    feedback_unit: String,
    setpoint_unit: String,
    proportional_gain_unit: String,
    integral_gain_unit: String,
    output_unit: String,
}

fn read_pi_controller_info(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    let setpoint = cursor.read_f64_le();
    let proportional_gain = cursor.read_f64_le();
    let integral_gain = cursor.read_f64_le();
    let lower_bound = cursor.read_f64_le();
    let upper_bound = cursor.read_f64_le();
    _ = cursor.read_u32_le();
    let feedback_unit = read_sm4_string(cursor);
    let setpoint_unit = read_sm4_string(cursor);
    let proportional_gain_unit = read_sm4_string(cursor);
    let integral_gain_unit = read_sm4_string(cursor);
    let output_unit = read_sm4_string(cursor);
    ReadType::PiControllerInfo(PiControllerInfo {
        setpoint,
        proportional_gain,
        integral_gain,
        lower_bound,
        upper_bound,
        feedback_unit,
        setpoint_unit,
        proportional_gain_unit,
        integral_gain_unit,
        output_unit,
    })
}

#[derive(Debug)]
struct LowpassFilterInfo {
    info: String,
}

fn read_lowpass_filter_info(cursor: &mut Cursor<&[u8]>, offset: u32) -> ReadType {
    cursor.set_position(offset as u64);
    _ = cursor.read_u32_le();
    let lowpass_filter_info = read_sm4_string(cursor);
    ReadType::LowpassFilterInfo(LowpassFilterInfo {
        info: lowpass_filter_info,
    })
}
