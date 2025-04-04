use phf::phf_map;

pub static INTERNAL_ERROR_CODES: phf::Map<&'static str, &'static str> = phf_map! {
    "GET_ME" => "1",
};
