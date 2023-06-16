use anyhow::Result;
use std::{fs::read, io::Cursor};

use crate::utils::Bytereading;

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
    id: u32,
    name: String,
    offset: u32,
    size: u32,
}

#[derive(Debug)]
struct PageIndexHeader {
    offset: u32,
    page_count: u32,
    object_list_count: u32,
}

#[derive(Debug)]
struct Sm4Page {
    page_id: u16,
    page_data_type: u32,
    page_data_type_name: String,
    page_source_type: u32,
    page_source_type_name: String,
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
    page_type: u32,
    page_type_name: String,
    data_sub_source: u32,
    line_type: u32,
    line_type_name: String,
    x_corner: u32,
    y_corner: u32,
    // xres
    x_size: u32,
    y_size: u32,
    image_type: u32,
    image_type_name: String,
    scan_type: u32,
    scan_type_name: String,
    group_id: u32,
    page_data_size: u32,
    min_z_value: u32,
    max_z_value: u32,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    xy_scale: f32,
    x_offset: f32,
    y_offset: f32,
    z_offset: f32,
    period: f32,
    bias: f32,
    current: f32,
    angle: f32,
    color_info_count: u32,
    grid_x_size: u32,
    grid_y_size: u32,
    object_list_count: u32,
    _32_bit_data_flag: u8,
    object_list: Vec<Sm4Object>,
    tiptrack_info_count: u32,
}

pub fn read_rhk_sm4(filename: &str) -> Result<()> {
    let bytes = read(filename)?;
    let _file_len = bytes.len();
    let mut cursor = Cursor::new(bytes.as_slice());

    let mut header = read_header(&mut cursor);
    // dbg!(&header);
    // cursor.set_position(size as u64);
    // dbg!(&cursor.position());

    // let mut header_object_list = Vec::with_capacity(header.object_list_count as usize);
    for _ in 0..header.object_list_count {
        header.object_list.push(read_sm4_object(&mut cursor))
    }
    // dbg!(&header_object_list);

    let page_index_header = get_page_index_header(&mut cursor, &header.object_list)?;
    // dbg!(&page_index_header);

    let mut page_index_header_list =
        Vec::with_capacity(page_index_header.object_list_count as usize);
    for _ in 0..page_index_header.object_list_count {
        page_index_header_list.push(read_sm4_object(&mut cursor))
    }
    // dbg!(&page_index_header_list);

    let page_index_array_offset = get_offset_page_index_array(&page_index_header_list)?;
    cursor.set_position(page_index_array_offset as u64);

    let mut pages = Vec::with_capacity(page_index_header.page_count as usize);
    for _ in 0..page_index_header.page_count {
        let mut page = read_sm4_page(&mut cursor);
        // dbg!(&page);

        for _ in 0..page.object_list_count {
            page.object_list.push(read_sm4_object(&mut cursor));
        }

        pages.push(page);
    }
    // dbg!(&pages);

    for page in pages {
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
        // dbg!(&page_header);
        for obj in &page.object_list {
            // dbg!(obj);
            let mut tiptrack_info_count = 0;
            if obj.offset != 0 && obj.size != 0 {
                match obj.id {
                    5 => read_image_drift_header(&mut cursor, obj.offset),
                    6 => read_image_drift(&mut cursor, obj.offset),
                    7 => read_spec_drift_header(&mut cursor, obj.offset),
                    8 => {
                        if let Sm4PageHeader::Default(ph) = page_header {
                            read_spec_drift_data(&mut cursor, obj.offset, ph.y_size);
                        };
                    }
                    9 => {}
                    10 => {
                        if let Sm4PageHeader::Default(ph) = page_header {
                            read_string_data(&mut cursor, obj.offset, ph.string_count);
                        };
                    }
                    11 => {
                        let tiptrack_header = read_tip_track_header(&mut cursor, obj.offset);
                        tiptrack_info_count = tiptrack_header.tiptrack_tiptrack_info_count;
                    }
                    12 => read_tip_track_data(&mut cursor, obj.offset, tiptrack_info_count),
                    13 => {}
                    // stopped here
                    15 => read_prm_header(&mut cursor, obj.offset),
                    17 => read_api_info(&mut cursor, obj.offset),
                    18 => read_history_info(&mut cursor, obj.offset),
                    19 => read_piezo_sensitivity(&mut cursor, obj.offset),
                    20 => read_frequency_sweep_data(&mut cursor, obj.offset),
                    21 => read_scan_processor_info(&mut cursor, obj.offset),
                    22 => read_pll_info(&mut cursor, obj.offset),

                    23 => read_channel_drive_info(&mut cursor, obj.offset),
                    24 => read_channel_drive_info(&mut cursor, obj.offset),

                    25 => read_lockin_info(&mut cursor, obj.offset),
                    26 => read_lockin_info(&mut cursor, obj.offset),

                    27 => read_pi_controller_info(&mut cursor, obj.offset),
                    28 => read_pi_controller_info(&mut cursor, obj.offset),
                    29 => read_pi_controller_info(&mut cursor, obj.offset),

                    29 => read_low_pass_filter_info(&mut cursor, obj.offset),
                    30 => read_low_pass_filter_info(&mut cursor, obj.offset),
                }
            }
        }
    }

    // dbg!(&cursor.position());

    Ok(())
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

fn get__drift_option_type_name(drift_option_type: u32) -> String {
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
        object_list_count,
    })
}

fn get_offset_page_index_header(object_list: &Vec<Sm4Object>) -> Result<u32> {
    for obj in object_list {
        if obj.name == "RHK_OBJECT_PAGE_INDEX_HEADER" {
            return Ok(obj.offset);
        }
    }
    Err(anyhow::anyhow!("No page index header"))
}

fn get_offset_page_index_array(object_list: &Vec<Sm4Object>) -> Result<u32> {
    for obj in object_list {
        if obj.name == "RHK_OBJECT_PAGE_INDEX_ARRAY" {
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
    let object_type_id = cursor.read_u32_le();
    let name = get_object_type_name(object_type_id);
    let offset = cursor.read_u32_le();
    let size = cursor.read_u32_le();

    Sm4Object {
        id: object_type_id,
        name,
        offset,
        size,
    }
}

fn read_sm4_page(cursor: &mut Cursor<&[u8]>) -> Sm4Page {
    // dbg!(&cursor.position());
    let page_id = cursor.read_u16_le();
    cursor.skip(14);
    let page_data_type = cursor.read_u32_le();
    let page_data_type_name = get_page_data_type_name(page_data_type);

    let page_source_type = cursor.read_u32_le();
    let page_source_type_name = get_page_source_type_name(page_source_type);

    let object_list_count = cursor.read_u32_le();

    let minor_version = cursor.read_u32_le();

    Sm4Page {
        page_id,
        page_data_type,
        page_data_type_name,
        page_source_type,
        page_source_type_name,
        minor_version,
        object_list_count,
        object_list: Vec::with_capacity(object_list_count as usize),
    }
}

fn read_page_header(cursor: &mut Cursor<&[u8]>, page: &Sm4Page) -> Result<Sm4PageHeader> {
    let offset = get_offset_object_page_header(&page.object_list)?;
    cursor.set_position(offset as u64);
    // Sequential data type
    if page.page_data_type == 6 {
        return Ok(Sm4PageHeader::Sequential(read_sequential_type(
            cursor, page,
        )));
    }
    Ok(Sm4PageHeader::Default(read_default_type(cursor, page)))
}

fn get_offset_object_page_header(object_list: &Vec<Sm4Object>) -> Result<u32> {
    for obj in object_list {
        if obj.name == "RHK_OBJECT_PAGE_HEADER" {
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
    let page_type = cursor.read_u32_le();
    let page_type_name = get_page_type_name(page_type);
    let data_sub_source = cursor.read_u32_le();

    let line_type = cursor.read_u32_le();
    let line_type_name = get_line_type_name(line_type);

    let x_corner = cursor.read_u32_le();
    let y_corner = cursor.read_u32_le();
    // xres
    let x_size = cursor.read_u32_le();
    let y_size = cursor.read_u32_le();

    let image_type = cursor.read_u32_le();
    let image_type_name = get_image_type_name(image_type);

    let scan_type = cursor.read_u32_le();
    let scan_type_name = get_scan_type_name(scan_type);

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
        page_type_name,
        data_sub_source,
        line_type,
        line_type_name,
        x_corner,
        y_corner,
        x_size,
        y_size,
        image_type,
        image_type_name,
        scan_type,
        scan_type_name,
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

struct ImageDriftHeader {
    imagedrift_filetime: u64,
    imagedrift_drift_option_type: u32,
    imagedrift_drift_option_type_name: String,
}

struct ImageDriftData {
    imagedrift_time: u32,
    imagedrift_dx: u32,
    imagedrift_dy: u32,
    imagedrift_cumulative_x: u32,
    imagedrift_cumulative_y: u32,
    imagedrift_vector_x: u32,
    imagedrift_vector_y: u32,
}

struct SpecDriftHeader {
    specdrift_filetime: u64,
    specdrift_drift_option_type: u32,
    specdrift_drift_option_type_name: String,
    specdrift_channel: String,
}

struct SpecDriftData {
    specdrift_time: Vec<f32>,
    specdrift_x_coord: Vec<f32>,
    specdrift_y_coord: Vec<f32>,
    specdrift_dx: Vec<f32>,
    specdrift_dy: Vec<f32>,
    specdrift_cumulative_x: Vec<f32>,
    specdrift_cumulative_y: Vec<f32>,
}

fn read_image_drift_header(cursor: &mut Cursor<&[u8]>, offset: u32) -> ImageDriftHeader {
    cursor.set_position(offset as u64);
    // unix epoch
    let imagedrift_filetime = cursor.read_u64_le();
    let imagedrift_drift_option_type = cursor.read_u32_le();
    let imagedrift_drift_option_type_name =
        get__drift_option_type_name(imagedrift_drift_option_type);
    ImageDriftHeader {
        imagedrift_filetime,
        imagedrift_drift_option_type,
        imagedrift_drift_option_type_name,
    }
}

fn read_image_drift(cursor: &mut Cursor<&[u8]>, offset: u32) -> ImageDriftData {
    cursor.set_position(offset as u64);
    let imagedrift_time = cursor.read_u32_le();
    let imagedrift_dx = cursor.read_u32_le();
    let imagedrift_dy = cursor.read_u32_le();
    let imagedrift_cumulative_x = cursor.read_u32_le();
    let imagedrift_cumulative_y = cursor.read_u32_le();
    let imagedrift_vector_x = cursor.read_u32_le();
    let imagedrift_vector_y = cursor.read_u32_le();
    ImageDriftData {
        imagedrift_time,
        imagedrift_dx,
        imagedrift_dy,
        imagedrift_cumulative_x,
        imagedrift_cumulative_y,
        imagedrift_vector_x,
        imagedrift_vector_y,
    }
}

fn read_spec_drift_header(cursor: &mut Cursor<&[u8]>, offset: u32) -> SpecDriftHeader {
    cursor.set_position(offset as u64);
    // unix epoch
    let specdrift_filetime = cursor.read_u64_le();
    let specdrift_drift_option_type = cursor.read_u32_le();
    let specdrift_drift_option_type_name = get__drift_option_type_name(specdrift_drift_option_type);
    _ = cursor.read_u32_le();
    let specdrift_channel = read_sm4_string(&mut cursor);

    SpecDriftHeader {
        specdrift_filetime,
        specdrift_drift_option_type,
        specdrift_drift_option_type_name,
        specdrift_channel,
    }
}

fn read_spec_drift_data(cursor: &mut Cursor<&[u8]>, offset: u32, y_size: u32) -> SpecDriftData {
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
    SpecDriftData {
        specdrift_time,
        specdrift_x_coord,
        specdrift_y_coord,
        specdrift_dx,
        specdrift_dy,
        specdrift_cumulative_x,
        specdrift_cumulative_y,
    }
}

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

fn read_string_data(cursor: &mut Cursor<&[u8]>, offset: u32, string_count: u16) -> StringData {
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
    StringData {
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
    }
}

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

fn read_tip_track_header(cursor: &mut Cursor<&[u8]>, offset: u32) -> TipTrackHeader {
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
    TipTrackHeader {
        tiptrack_filetime,
        tiptrack_feature_height,
        tiptrack_feature_width,
        tiptrack_time_constant,
        tiptrack_cycle_rate,
        tiptrack_phase_lag,
        tiptrack_tiptrack_info_count,
        tiptrack_channel,
    }
}

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
) -> TipTrackData {
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
    TipTrackData {
        tiptrack_cumulative_time,
        tiptrack_time,
        tiptrack_dx,
        tiptrack_dy,
    }
}
